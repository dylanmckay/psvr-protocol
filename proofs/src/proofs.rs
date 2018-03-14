use hmdee::backend::psvr;

pub fn errors_are_returned_as_ascii_strings(psvr: &mut psvr::Psvr) {
    psvr.send_command(&psvr::command::SetCinematicConfiguration {
        mask: 0xc0,
        screen_size: 32,
        screen_distance: 23,
        ipd: 20,
        reserved0: [0; 6],
        brightness: 20,
        mic_volume: 22,
        reserved1: [22; 2],
        unknown: false,
        reserved2: 1,
    }).unwrap();

    for i in 0..100 {
        psvr.send_command_raw(&[0x81, 0x0, 0xaa, 0x8, 0x80, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]).unwrap();

        let _ = psvr.receive_control(i).unwrap();
    }
}

