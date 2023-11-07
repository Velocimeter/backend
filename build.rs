use ethers::prelude::Abigen;

fn main() {
    rust_file_generation("ERC20", "./abi/ERC20.json", "./src/bindings/erc20.rs");
    rust_file_generation("oTOKEN", "./abi/oTOKEN.json", "./src/bindings/otoken.rs");
    rust_file_generation("Gauge", "./abi/Gauge.json", "./src/bindings/gauge.rs");
    rust_file_generation("Factory", "./abi/Factory.json", "./src/bindings/factory.rs");
    rust_file_generation(
        "RewardsDistributer",
        "./abi/RewardsDistributer.json",
        "./src/bindings/rewards_distributer.rs",
    );
    rust_file_generation("Router", "./abi/Router.json", "./src/bindings/router.rs");
    rust_file_generation("Voter", "./abi/Voter.json", "./src/bindings/voter.rs");
    rust_file_generation(
        "VotingEscrow",
        "./abi/VotingEscrow.json",
        "./src/bindings/voting_escrow.rs",
    );
    rust_file_generation("Pair", "./abi/Pair.json", "./src/bindings/pair.rs");
    rust_file_generation("Minter", "./abi/Minter.json", "./src/bindings/minter.rs");
    rust_file_generation("Bribe", "./abi/Bribe.json", "./src/bindings/bribe.rs");
}

fn rust_file_generation(name: &str, abi_source: &str, path: &str) {
    let out_file = std::env::current_dir()
        .expect("Could not get current dir")
        .join(path);
    if out_file.exists() {
        std::fs::remove_file(&out_file).expect("Could not remove file");
    }
    Abigen::new(name, abi_source)
        .expect("Could not create Abigen")
        .generate()
        .expect("Could not generate abigen")
        .write_to_file(out_file)
        .expect("could not write bindings to file");
}
