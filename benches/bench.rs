#![feature(test)]
extern crate test;
extern crate mqtt;

use test::Bencher;

use mqtt::*;

#[bench]
fn bench_decode_connect_packets(b: &mut Bencher) {
    let buf = b"\x10\x1D\x00\x04MQTT\x04\xC0\x00\x3C\x00\
\x0512345\x00\x04user\x00\x04pass";

    b.iter(|| decode_packet(buf).unwrap());
}

#[bench]
fn bench_decode_connect_ack_packets(b: &mut Bencher) {
    let buf = b"\x20\x02\x01\x04";

    b.iter(|| decode_packet(buf).unwrap());
}

#[bench]
fn bench_decode_publish_packets(b: &mut Bencher) {
    let buf = b"\x3d\x0D\x00\x05topic\x43\x21data";

    b.iter(|| decode_packet(buf).unwrap());
}

#[bench]
fn bench_decode_subscribe_packets(b: &mut Bencher) {
    let buf = b"\x82\x12\x12\x34\x00\x04test\x01\x00\x06filter\x02";

    b.iter(|| decode_packet(buf).unwrap());
}

#[bench]
fn bench_decode_subscribe_ack_packets(b: &mut Bencher) {
    let buf = b"\x90\x05\x12\x34\x01\x80\x02";

    b.iter(|| decode_packet(buf).unwrap());
}

#[bench]
fn bench_decode_unsubscribe_packets(b: &mut Bencher) {
    let buf = b"\xa2\x10\x12\x34\x00\x04test\x00\x06filter";

    b.iter(|| decode_packet(buf).unwrap());
}

#[bench]
fn bench_encode_connect_packets(b: &mut Bencher) {
    let p = Packet::Connect {
        clean_session: false,
        keep_alive: 60,
        client_id: b"12345",
        will: Some(ConnectionWill {
            qos: QoS::ExactlyOnce,
            retain: false,
            topic: "topic",
            message: "message",
        }),
        username: None,
        password: None,
    };

    let mut v = Vec::new();

    b.iter(|| {
        v.clear();
        v.write_packet(&p).unwrap();
    });
}

#[bench]
fn bench_encode_publish_packets(b: &mut Bencher) {
    let p = Packet::Publish {
        dup: true,
        retain: true,
        qos: QoS::ExactlyOnce,
        topic: "topic",
        packet_id: Some(0x4321),
        payload: b"data",
    };

    let mut v = Vec::new();

    b.iter(|| {
        v.clear();
        v.write_packet(&p).unwrap();
    });
}

#[bench]
fn bench_encode_subscribe_packets(b: &mut Bencher) {
    let p = Packet::Subscribe {
        packet_id: 0x1234,
        topic_filters: vec![("test", QoS::AtLeastOnce), ("filter", QoS::ExactlyOnce)],
    };

    let mut v = Vec::new();

    b.iter(|| {
        v.clear();
        v.write_packet(&p).unwrap();
    });
}

#[bench]
fn bench_encode_subscribe_ack_packets(b: &mut Bencher) {
    let p = Packet::SubscribeAck {
        packet_id: 0x1234,
        status: vec![SubscribeStatus::Success(QoS::AtLeastOnce),
                     SubscribeStatus::Failure,
                     SubscribeStatus::Success(QoS::ExactlyOnce)],
    };

    let mut v = Vec::new();

    b.iter(|| {
        v.clear();
        v.write_packet(&p).unwrap();
    });
}

#[bench]
fn bench_encode_unsubscribe_packets(b: &mut Bencher) {
    let p = Packet::Unsubscribe {
        packet_id: 0x1234,
        topic_filters: vec!["test", "filter"],
    };

    let mut v = Vec::new();

    b.iter(|| {
        v.clear();
        v.write_packet(&p).unwrap();
    });
}