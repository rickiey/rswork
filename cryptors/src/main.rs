use x25519_dalek::{EphemeralSecret,StaticSecret, PublicKey};
use hex::{encode,decode};

fn main() {
  
    ecdh()
}


pub fn ecdh() {
    // let alice_secret = StaticSecret::random();
    // let alice_public = PublicKey::from(&alice_secret);

    // let bob_secret = StaticSecret::random();
    // let bob_public = PublicKey::from(&bob_secret);

    // //     # use rand_core::OsRng;
    // // # use x25519_dalek::{EphemeralSecret, PublicKey};
    // // # let alice_secret = EphemeralSecret::new(OsRng);
    // // # let alice_public = PublicKey::from(&alice_secret);
    // // # let bob_secret = EphemeralSecret::new(OsRng);
    // // # let bob_public = PublicKey::from(&bob_secret);
    // let alice_shared_secret = alice_secret.diffie_hellman(&bob_public);
    // let bob_shared_secret = bob_secret.diffie_hellman(&alice_public);

    // let s1 = encode(alice_shared_secret.to_bytes());
    // let s2 = encode(bob_shared_secret.to_bytes());

    // println!("alice_secret: {:?}", alice_secret.as_bytes());
    // println!("bob_secret: {:?}",bob_secret.to_bytes());
    // println!("s1: {}",s1);
    // println!("s2: {}",s2);


    let k1 = decode("562d726276b9e2dc0c3b8e74ae7b25c7af562152351d7d6ce4012a055fef1e15".as_bytes()).ok().unwrap();
    let k2 = decode("a300cc658fd46e840a932175a3c0253b58c09560b27ed4c491c399d8bb5dbeeb".as_bytes()).ok().unwrap();
    println!("k1 {:?}",k1) ;
    println!("k2 {:?}",k2) ;

    let u8k1 :Result<[u8;32],_>= k1.try_into() ;
    let u8k2 :Result<[u8;32],_>= k2.try_into() ;

    let ik1 = StaticSecret::from( u8k1.ok().unwrap()) ;
    let ik2 = StaticSecret::from( u8k2.ok().unwrap() ) ;

    let sks1 = ik1.diffie_hellman(&PublicKey::from(&ik2)) ;
    let sks2 = ik2.diffie_hellman(&PublicKey::from(&ik1)) ;

    println!("{:?}",encode(sks1.to_bytes())) ;
    println!("{:?}",encode(sks2.to_bytes())) ;
    // eb76946b432243b266f807a602596069b4b85a313a7c65649f00a68fec9d9374

}