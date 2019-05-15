mod huffman;

fn main() {
    let text = String::from("h e e  hh e");
    println!("text before decoding : {}", text);
    let (mut encoded, ls, table) = huffman::encode(&text);
    // let decoded = huffman::decode(encoded);
    println!("Encoded text : {:?}", encoded);
    print!(
        "\n Decoded text : {:?}",
        huffman::decode(&mut encoded, ls, table)
    )
}
