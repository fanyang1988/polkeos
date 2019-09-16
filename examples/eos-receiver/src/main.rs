extern crate rustc_serialize;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct PermissionLevel {
    actor:String,
    permission:String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Action {
    account:String,
    name:String,
    authorization:Vec<PermissionLevel>
    // TODO: datas
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Transaction  {
    expiration:String,
    ref_block_num:u16,
    ref_block_prefix:u32,
    max_net_usage_words:u32,
    max_cpu_usage_ms:u8,
    delay_sec:u32,

    context_free_actions:Vec<Action>,
    actions:Vec<Action>,
    // TODO: transaction_extensions:Vec<Extension>

    signatures:Vec<String>,
    context_free_data:Vec<Vec<u8>>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct TransactionReceipt {
    status: String,
    cpu_usage_us:u32,
    net_usage_words:u32,

    id:String,

    trx: Transaction,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct SignedBlockHeader {
    timestamp: String,
    producer: String,
    confirmed: u16,
    previous: String,
    transaction_mroot:String,
    action_mroot:String,
    schedule_version:u32,
    producer_signature:String,
    // TODO: ext data
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct BlockVerifyTrace {
    header_hash:String,
    schedule_producer_hash:String,
    sig_digest:String,
    blockroot_merkle:String,
    producer_key:String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct TransferActionData {
    from:String,
    to:String,
    amount:i64,
    precision:u8,
    symbol:String,
    memo:String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct KeyActions {
    transfers:Vec<TransferActionData>
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct BlockTraceData  {
    id:String,
    num:u32,
    header: SignedBlockHeader,
    verify: BlockVerifyTrace,
    key_actions:KeyActions,
    transactions:Vec<TransactionReceipt>
}

fn main() {
    let context = zmq::Context::new();
    let router = context.socket(zmq::ROUTER).unwrap();
    assert!(router.bind("tcp://*:15555").is_ok());
    loop {
        let identity = router.recv_bytes(0).unwrap();
        assert!(!identity.is_empty());

        let block_str = router.recv_string(0).unwrap().unwrap();
        let brd: BlockTraceData = json::decode(&block_str).unwrap();
        println!("block received, num: {}, id: {}, trx size: {}\n", brd.num, brd.id, brd.transactions.len());
        for act in brd.key_actions.transfers{
            println!("transfer from: {}, to: {}, q: {} {},{}, memo: {}\n", 
                act.from, act.to, act.amount, act.precision, act.symbol, act.memo)
        }
    }
}
