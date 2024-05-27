pub mod solidity;
pub mod tinyvm;
use crate::solidity::grammar::*;
use crate::tinyvm::*;

fn main() {
    let code =
        std::fs::read_to_string("./contracts/empty.sol").expect("Unable to read source file");
    let parsed = parse(code.as_str());
    println!("{:#?}", parsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let code =
            std::fs::read_to_string("./contracts/empty.sol").expect("Unable to read source file");
        let parsed = parse(code.as_str());
        println!("{:#?}", parsed);
        assert!(parsed.is_ok());
    }
    #[test]
    fn test_parse_empty_contract() {
        let code = std::fs::read_to_string("./contracts/empty_contract.sol")
            .expect("Unable to read source file");
        let parsed = parse(code.as_str());
        println!("{:#?}", parsed);
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_parse_dummy_contract_with_var() {
        let code = std::fs::read_to_string("./contracts/contract_with_var.sol")
            .expect("Unable to read source file");
        let parsed = parse(code.as_str());
        println!("{:#?}", parsed);
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_parse_flipper_contract() {
        let code =
            std::fs::read_to_string("./contracts/flipper.sol").expect("Unable to read source file");
        let parsed = parse(code.as_str());
        println!("{:#?}", parsed);
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_call_flipper_contract_flip() {
        let code =
            std::fs::read_to_string("./contracts/flipper.sol").expect("Unable to read source file");
        let parsed = parse(code.as_str());
        println!("{:#?}", parsed);
        assert!(parsed.is_ok());

        match parsed {
            Ok(source_unit) => {
                let contracts = create_contracts(source_unit);
                let contract = contracts.first();
                assert!(contract.is_some());

                let flip_func_sig = get_func_sig("flip()".to_string());
                let mutated_contract = contract.unwrap().call(flip_func_sig.as_str()).0;

                let get_func_sig = get_func_sig("get()".to_string());
                let ret = mutated_contract.call(get_func_sig.as_str()).1;
                match ret.as_slice() {
                    [solidity::grammar::Expression::BoolLiteral(val)] => {
                        assert_eq!(*val, true);
                    }
                    _ => assert!(false, "Unexpected return value: {:?}", ret),
                }
                println!("Return value: {:?}", ret);
            }
            Err(e) => assert!(false, "Error: {:?}", e),
        }
    }
}
