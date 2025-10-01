use evm::tracing::{Event, EventListener};
use evm::ExitReason;
use protocol::types::{Bytes, CallFrame, CallType};

/// Call tracer that implements EventListener to capture execution trace
#[derive(Default, Clone, Debug)]
pub struct CallTracer {
    /// Stack of call frames, where the last element is the current executing
    /// frame
    call_stack:  Vec<CallFrame>,
    /// Whether we've seen the initial transaction event
    initialized: bool,
    /// Root frame captured when execution completes
    root:        Option<CallFrame>,
}

impl CallTracer {
    pub fn new() -> Self {
        Self {
            call_stack:  Vec::new(),
            initialized: false,
            root:        None,
        }
    }

    /// Extract the root call frame after execution completes
    pub fn into_result(self) -> Option<CallFrame> {
        if !self.call_stack.is_empty() {
            return self.call_stack.into_iter().next();
        }

        self.root
    }

    /// Push a new call frame onto the stack
    fn push_frame(&mut self, frame: CallFrame) {
        self.call_stack.push(frame);
    }

    /// Pop the current frame and attach it to its parent, or keep as root
    fn pop_frame(&mut self, return_value: &[u8], reason: &ExitReason) {
        if let Some(mut frame) = self.call_stack.pop() {
            frame.on_exit(return_value, reason);
            frame.gas_used = None;

            // Attach to parent or store as root
            if let Some(parent) = self.call_stack.last_mut() {
                parent.add_call(frame);
            } else {
                self.root = Some(frame);
            }
        }
    }
}

impl EventListener for CallTracer {
    fn event(&mut self, event: Event) {
        match event {
            // Transaction-level call (root call)
            Event::TransactCall {
                caller,
                address,
                value,
                data,
                gas_limit,
            } => {
                if !self.initialized {
                    let frame = CallFrame::new(
                        CallType::Call,
                        caller,
                        Some(address),
                        value,
                        gas_limit,
                        Bytes::from(data.to_vec()),
                    );
                    self.push_frame(frame);
                    self.initialized = true;
                }
            }

            // Transaction-level create (root create)
            Event::TransactCreate {
                caller,
                value,
                init_code,
                address,
                gas_limit,
            } => {
                if !self.initialized {
                    let frame = CallFrame::new(
                        CallType::Create,
                        caller,
                        Some(address),
                        value,
                        gas_limit,
                        Bytes::from(init_code.to_vec()),
                    );
                    self.push_frame(frame);
                    self.initialized = true;
                }
            }

            // Transaction-level create2
            Event::TransactCreate2 {
                caller,
                value,
                init_code,
                salt: _,
                address,
                gas_limit,
            } => {
                if !self.initialized {
                    let frame = CallFrame::new(
                        CallType::Create2,
                        caller,
                        Some(address),
                        value,
                        gas_limit,
                        Bytes::from(init_code.to_vec()),
                    );
                    self.push_frame(frame);
                    self.initialized = true;
                }
            }

            // Internal call
            Event::Call {
                code_address,
                transfer,
                input,
                target_gas,
                is_static,
                context,
            } => {
                let from = context.caller;
                let to = Some(code_address);
                let value = transfer.as_ref().map(|t| t.value).unwrap_or_default();
                let gas = target_gas.unwrap_or(0);

                // Determine call type based on transfer and is_static
                let call_type = if is_static {
                    CallType::StaticCall
                } else if transfer.is_none() {
                    // No value transfer might indicate DELEGATECALL
                    CallType::DelegateCall
                } else {
                    CallType::Call
                };

                let frame =
                    CallFrame::new(call_type, from, to, value, gas, Bytes::from(input.to_vec()));
                self.push_frame(frame);
            }

            // Internal create
            Event::Create {
                caller,
                address,
                scheme,
                value,
                init_code,
                target_gas,
            } => {
                let gas = target_gas.unwrap_or(0);
                let call_type = match scheme {
                    evm::CreateScheme::Create2 { .. } => CallType::Create2,
                    _ => CallType::Create,
                };

                let frame = CallFrame::new(
                    call_type,
                    caller,
                    Some(address),
                    value,
                    gas,
                    Bytes::from(init_code.to_vec()),
                );
                self.push_frame(frame);
            }

            // Precompile subcall
            Event::PrecompileSubcall {
                code_address,
                transfer,
                input,
                target_gas,
                is_static,
                context,
            } => {
                // Handle precompile subcalls similarly to normal calls
                let from = context.caller;
                let to = Some(code_address);
                let value = transfer.as_ref().map(|t| t.value).unwrap_or_default();
                let gas = target_gas.unwrap_or(0);

                let call_type = if is_static {
                    CallType::StaticCall
                } else {
                    CallType::Call
                };

                let frame =
                    CallFrame::new(call_type, from, to, value, gas, Bytes::from(input.to_vec()));
                self.push_frame(frame);
            }

            // Self-destruct
            Event::Suicide {
                address,
                target,
                balance,
            } => {
                if let Some(parent) = self.call_stack.last_mut() {
                    parent.add_call(CallFrame::new(
                        CallType::SelfDestruct,
                        address,
                        Some(target),
                        balance,
                        0,
                        Bytes::new(),
                    ));
                }
            }

            // Exit from a call/create
            Event::Exit {
                reason,
                return_value,
            } => {
                self.pop_frame(return_value, reason);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use evm::{ExitReason, ExitSucceed};
    use protocol::types::{Bytes, CallFrame, CallType, H160, U256};

    use super::*;

    #[test]
    fn test_call_frame_creation() {
        let frame = CallFrame::new(
            CallType::Call,
            H160::zero(),
            Some(H160::zero()),
            U256::zero(),
            21000,
            Bytes::new(),
        );

        assert_eq!(frame.type_, CallType::Call);
        assert_eq!(frame.gas, 21000);
        assert_eq!(frame.calls.len(), 0);
    }

    #[test]
    fn test_call_type_string() {
        assert_eq!(CallType::Call.as_str(), "CALL");
        assert_eq!(CallType::StaticCall.as_str(), "STATICCALL");
        assert_eq!(CallType::Create.as_str(), "CREATE");
        assert_eq!(CallType::Create2.as_str(), "CREATE2");
    }

    #[test]
    fn test_tracer_initialization() {
        let tracer = CallTracer::new();
        assert_eq!(tracer.call_stack.len(), 0);
        assert!(!tracer.initialized);
    }

    #[test]
    fn test_tracer_simple_call() {
        let mut tracer = CallTracer::new();

        // Simulate a transaction call
        tracer.event(Event::TransactCall {
            caller:    H160::zero(),
            address:   H160::from_low_u64_be(1),
            value:     U256::from(100),
            data:      &[1, 2, 3],
            gas_limit: 50000,
        });

        assert_eq!(tracer.call_stack.len(), 1);
        assert!(tracer.initialized);

        // Simulate exit
        tracer.event(Event::Exit {
            reason:       &ExitReason::Succeed(ExitSucceed::Returned),
            return_value: &[4, 5, 6],
        });

        let result = tracer.into_result();
        assert!(result.is_some());

        let root = result.unwrap();
        assert_eq!(root.from, H160::zero());
        assert_eq!(root.to, Some(H160::from_low_u64_be(1)));
        assert_eq!(root.value, U256::from(100));
        assert!(root.error.is_none());
    }

    #[test]
    fn test_tracer_nested_calls() {
        let mut tracer = CallTracer::new();

        // Root call
        tracer.event(Event::TransactCall {
            caller:    H160::zero(),
            address:   H160::from_low_u64_be(1),
            value:     U256::from(100),
            data:      &[],
            gas_limit: 100000,
        });

        // Nested call
        let transfer = evm::Transfer {
            source: H160::from_low_u64_be(1),
            target: H160::from_low_u64_be(2),
            value:  U256::from(50),
        };
        let context = evm::Context {
            address:        H160::from_low_u64_be(2),
            caller:         H160::from_low_u64_be(1),
            apparent_value: U256::from(50),
        };

        tracer.event(Event::Call {
            code_address: H160::from_low_u64_be(2),
            transfer:     &Some(transfer),
            input:        &[7, 8, 9],
            target_gas:   Some(50000),
            is_static:    false,
            context:      &context,
        });

        // Exit nested call
        tracer.event(Event::Exit {
            reason:       &ExitReason::Succeed(ExitSucceed::Returned),
            return_value: &[10, 11],
        });

        // Exit root call
        tracer.event(Event::Exit {
            reason:       &ExitReason::Succeed(ExitSucceed::Returned),
            return_value: &[12, 13],
        });

        let result = tracer.into_result();
        assert!(result.is_some());

        let root = result.unwrap();
        assert_eq!(root.calls.len(), 1);
        assert_eq!(root.calls[0].from, H160::from_low_u64_be(1));
        assert_eq!(root.calls[0].to, Some(H160::from_low_u64_be(2)));
    }
}
