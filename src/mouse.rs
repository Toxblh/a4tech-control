use rusb::*;
use std::time::Duration;

const A4TECH_VID: u16 = 0x09da;
const BLOODY_V5_PID: u16 = 0x172A;
const BLOODY_V7_PID: u16 = 0xF613;
const BLOODY_V7M_PID: u16 = 0x3AC4;
const BLOODY_V8_PID: u16 = 0x11F5;
const BLOODY_V8M_PID: u16 = 0x5650;
const BLOODY_R7_PID: u16 = 0x1485;
const BLOODY_R8_1_PID: u16 = 0x14ee;
const BLOODY_R3_PID: u16 = 0x1a5a;
const BLOODY_AL9_PID: u16 = 0xf633;
const BLOODY_R70_PID: u16 = 0xf643;
const BLOODY_A7_PID: u16 = 0x7e36;
const BLOODY_A9_PID: u16 = 0x1003;
const COMPATIBLE_PIDS: [u16; 12] = [
    BLOODY_V5_PID,
    BLOODY_V7_PID,
    BLOODY_V7M_PID,
    BLOODY_V8_PID,
    BLOODY_V8M_PID,
    BLOODY_R7_PID,
    BLOODY_R8_1_PID,
    BLOODY_R3_PID,
    BLOODY_AL9_PID,
    BLOODY_R70_PID,
    BLOODY_A7_PID,
    BLOODY_A9_PID,
];
const A4TECH_MAGIC: u8 = 0x07;

const DPI_OPCODE: u8 = 0x0d;
// const INFO_OPCODE: u8 = 0x05;

// const BREATH_OPCODE: u8 = 0x03;
// const BREATH_OPCODE1: u8 = 0x06;
// const BREATH_OPCODE2: u8 = 0x01;

const BRIGHTNESS_OPCODE: u8 = 0x11;
const BRIGHTNESS_WRITE: u8 = 0x80;
const BRIGHTNESS_READ: u8 = 0x00;

pub fn get_name(id_product: u16) -> &'static str {
    match id_product {
        BLOODY_V5_PID => "Bloody V5",
        BLOODY_V7_PID => "Bloody V7",
        BLOODY_V7M_PID => "Bloody V7M",
        BLOODY_V8_PID => "Bloody V8",
        BLOODY_V8M_PID=> "Bloody V8M",
        BLOODY_R7_PID => "Bloody R7",
        BLOODY_R8_1_PID => "Bloody R8-1",
        BLOODY_R3_PID => "Bloody R3",
        BLOODY_AL9_PID => "Bloody AL9",
        BLOODY_R70_PID => "Bloody R70",
        BLOODY_A7_PID => "Bloody A7",
        BLOODY_A9_PID => "Bloody A9",
        _ => "Unknown",
    }
}

fn open_device<T: UsbContext>(context: &mut T) -> Option<(DeviceHandle<T>, rusb::DeviceDescriptor)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == A4TECH_VID {
            for &x in COMPATIBLE_PIDS.iter() {
                if device_desc.product_id() == x {
                    // println!("{:?}", device_desc);
                    println!("Found! {}", get_name(device_desc.product_id()));

                    match device.open() {
                        Ok(mut handle) => {
                            match handle.kernel_driver_active(2 as u8) {
                                Ok(res) => {
                                    if res {
                                        match handle.detach_kernel_driver(2 as u8) {
                                            Ok(_) => {}
                                            Err(_) => continue,
                                        }
                                    }
                                }
                                Err(e) => println!("Erroe {}", e),
                            }

                            return Some((handle, device_desc));
                        }
                        Err(_) => continue,
                    }
                }
            }
        }
    }

    None
}

#[allow(dead_code)]
fn print_device_info<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<()> {
    let device_desc = handle.device().device_descriptor()?;
    let timeout = Duration::from_secs(1);
    let languages = handle.read_languages(timeout)?;

    println!("Active configuration: {}", handle.active_configuration()?);

    if !languages.is_empty() {
        let language = languages[0];
        println!("Language: {:?}", language);

        println!(
            "Manufacturer: {}",
            handle
                .read_manufacturer_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "Product: {}",
            handle
                .read_product_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "Serial Number: {}",
            handle
                .read_serial_number_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
    }
    Ok(())
}

pub fn read_brightness<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<u8> {
    let mut res: [u8; 72] = [0x00; 72];
    let mut data: [u8; 72] = [0x00; 72];
    data[0] = A4TECH_MAGIC;
    data[1] = BRIGHTNESS_OPCODE;
    data[4] = BRIGHTNESS_READ;

    read_from_mouse(handle, &data, &mut res).expect("read_brightness");

    for i in 0..res.len() {
        print!("{}:{:02x} ", i, res[i]);
    }

    println!("");

    println!("Brightness: {}", res[8]);

    Ok(res[8])
}

pub fn read_dpi<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<u8> {
    let mut res: [u8; 72] = [0x00; 72];
    let mut data: [u8; 72] = [0x00; 72];
    data[0] = A4TECH_MAGIC;
    data[1] = DPI_OPCODE;

    read_from_mouse(handle, &data, &mut res).expect("read_dpi");

    for i in 0..res.len() {
        print!("{:02x} ", res[i]);
    }

    println!("");

    println!("DPI: {}", res[8]);

    Ok(res[8])
}

pub fn write_brightness<T: UsbContext>(handle: &mut DeviceHandle<T>, level: u8) -> Result<()> {
    let mut data: [u8; 72] = [0x00; 72];
    data[0] = A4TECH_MAGIC;
    data[1] = BRIGHTNESS_OPCODE;
    data[4] = BRIGHTNESS_WRITE;
    data[8] = level;

    write_to_mouse(handle, &data).expect("write_brightness");

    Ok(())
}

fn write_to_mouse<T: UsbContext>(handle: &mut DeviceHandle<T>, data: &[u8]) -> Result<()> {
    match handle.write_control(0x21, 9, 0x0307, 2, &data, Duration::from_secs(10)) {
        Ok(res) => println!("Ok! {}", res),
        Err(e) => println!("Write to mouse Error! {}", e),
    }

    Ok(())
}

fn read_from_mouse<T: UsbContext>(
    handle: &mut DeviceHandle<T>,
    request_data: &[u8],
    mut response_data: &mut [u8],
) -> Result<()> {
    match write_to_mouse(handle, request_data) {
        Ok(_) => {
            match handle.read_control(
                0xa1,
                1,
                0x0307,
                2,
                &mut response_data,
                std::time::Duration::new(10, 0),
            ) {
                Ok(res) => println!("Ok! {}", res),
                Err(e) => println!("read_from_mouse Error! {}", e),
            }

            Ok(())
        }
        Err(e) => {
            println!("Error! {}", e);
            Ok(())
        }
    }
}

pub fn init() -> (rusb::DeviceHandle<rusb::Context>, rusb::DeviceDescriptor) {
    let mut context = Context::new().expect("Failed to get context");
    rusb::set_log_level(rusb::LogLevel::Info);
    let (handle, device) = open_device(&mut context).expect("Failed to open USB device");

    (handle, device)
}

pub fn test() {
    let (mut handle, _)= init();
    let level = read_brightness(&mut handle).unwrap();
    println!("Level: {}", level);

    read_dpi(&mut handle).unwrap();
}
