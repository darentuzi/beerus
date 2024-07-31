use iamgroot::jsonrpc;
use serde_json::Value;
use wiremock::{Match, Request};

pub trait Response {
    fn response(&self) -> Value;
}

pub struct ChainIdMatcher {
    pub response: Value,
}

impl Default for ChainIdMatcher {
    fn default() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0x4b4154414e41"
            }),
        }
    }
}

impl Response for ChainIdMatcher {
    fn response(&self) -> Value {
        self.response.clone()
    }
}

impl Match for ChainIdMatcher {
    fn matches(&self, request: &Request) -> bool {
        let request = request.body_json::<jsonrpc::Request>().unwrap();
        match request.method.as_str() {
            "starknet_chainId" => true,
            _ => false,
        }
    }
}

pub struct NonceMatcher {
    pub response: Value,
}

impl Default for NonceMatcher {
    fn default() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0x0"
            }),
        }
    }
}

impl Match for NonceMatcher {
    fn matches(&self, request: &Request) -> bool {
        let request = request.body_json::<jsonrpc::Request>().unwrap();
        match request.method.as_str() {
            "starknet_getNonce" => true,
            _ => false,
        }
    }
}

impl Response for NonceMatcher {
    fn response(&self) -> Value {
        self.response.clone()
    }
}

pub struct ClassMatcher {
    pub response: Value,
}

impl ClassMatcher {
    pub fn error() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "error": {
                    "code": 28,
                    "message": "Class hash not found"
                }
            }),
        }
    }

    pub fn success() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": {
                    "sierra_program": ["0x1"],
                    "contract_class_version": "0.1.0",
                    "entry_points_by_type": {
                        "CONSTRUCTOR": [{
                            "selector": "0x2",
                            "function_idx": 2,
                        }],
                        "EXTERNAL": [{
                            "selector": "0x3",
                            "function_idx": 3,
                        }, {
                            "selector": "0x4",
                            "function_idx": 4,
                        }],
                        "L1_HANDLER": [],
                    },
                    "abi": "some_abi"
                }
            }),
        }
    }
}

impl Match for ClassMatcher {
    fn matches(&self, request: &Request) -> bool {
        let request = request.body_json::<jsonrpc::Request>().unwrap();
        match request.method.as_str() {
            "starknet_getClass" => true,
            _ => false,
        }
    }
}

impl Response for ClassMatcher {
    fn response(&self) -> Value {
        self.response.clone()
    }
}

pub struct SpecVersionMatcher {
    pub response: Value,
}

impl Default for SpecVersionMatcher {
    fn default() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": "0.6.0"
            }),
        }
    }
}

impl Match for SpecVersionMatcher {
    fn matches(&self, request: &Request) -> bool {
        let request = request.body_json::<jsonrpc::Request>().unwrap();
        match request.method.as_str() {
            "starknet_specVersion" => true,
            _ => false,
        }
    }
}

impl Response for SpecVersionMatcher {
    fn response(&self) -> Value {
        self.response.clone()
    }
}

pub struct EstimateFeeMatcher {
    pub response: Value,
}

impl Default for EstimateFeeMatcher {
    fn default() -> Self {
        Self {
            response: serde_json::json!(
            {
                "jsonrpc": "2.0",
                "id": 1,
                "result": [{
                    "gas_consumed": "0x18bf",
                    "gas_price": "0x174876e800",
                    "overall_fee": "0x2402a36771800",
                    "unit": "WEI"
                }]
            }),
        }
    }
}

impl Match for EstimateFeeMatcher {
    fn matches(&self, request: &Request) -> bool {
        let request = request.body_json::<jsonrpc::Request>().unwrap();
        match request.method.as_str() {
            "starknet_estimateFee" => true,
            _ => false,
        }
    }
}

impl Response for EstimateFeeMatcher {
    fn response(&self) -> Value {
        self.response.clone()
    }
}
