use std::fmt::format;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

const SYS_GPIO_IN: i32 = 0;
const SYSFS_GPIO_OUT: i32 = 1;

const SYSFS_GPIO_LOW: i32 = 0;
const SYSFS_GPIO_HIGH: i32 = 1;

const  NUM_MAX_BUF: usize =  4;
const  DIR_MAX_SIZ: usize =  60;

fn snprintf(buffer: &mut [u8], format_str: &str, arg: i32) -> usize {
    let mut bytes_written = 0;

    let formatted_str = format!(format_str, arg);
    let formatted_bytes = formatted_str.as_bytes();

    for (i, byte) in formatted_bytes.iter().enumerate() {
        if i >= buffer.len() {
            break;
        }
        buffer[i] = *byte;
        bytes_written += 1;
    }

    if bytes_written < buffer.len() {
        buffer[bytes_written] = 0;
    }

    bytes_written
}

pub fn sysfs_gpio_export(pin:i32){
    let buff = Vec::with_capacity(NUM_MAX_BUF).as_mut_slice();
    let mut fd = File::open("/sys/class/gpio/export").expect(format!("Export Failed: Pin{} \n", pin).as_str());
    let len = match std::write(&mut buffer[..NUM_MAX_BUF], format_args!("{}", pin)) {
        Ok(len) => len,
        Err(_) => Err("[sysfs_gpio_export], Error writing to buffer"),
    };
    fd.write(&buff[..len]).unwrap();
}

pub fn sysfs_gpio_unexport(pin: i32) {
    let buff = Vec::with_capacity(NUM_MAX_BUF).as_mut_slice();
    let mut fd = File::open("/sys/class/gpio/unexport").expect(format!("Unexport Failed: Pin{} \n", pin).as_str());
    let len = match std::write(&mut buffer[..NUM_MAX_BUF], format_args!("{}", pin)) {
        Ok(len) => len,
        Err(_) => Err("[sysfs_gpio_unexport], Error writing to buffer"),
    };
    fd.write(&buff[..len]).unwrap();
}

pub fn sysfs_gpio_direction(pin: i32, dir: i32) {
    let dir_str = "in\0out";
    let path = Vec::with_capacity(DIR_MAX_SIZ).as_mut_slice();
    match std::write(&mut buffer[..DIR_MAX_SIZ], format_args!("{}", pin)) {
        Ok(len) => len,
        Err(_) => Err("[sysfs_gpio_direction], Error writing to buffer"),
    }.unwrap();
    let mut fd = File::open(path).expect(format!("Set Direction failed: Pin {}", pin).as_str());
    let mut to_write = &dir_str[3..];
    if dir == SYS_GPIO_IN {
        to_write = &dir_str[0..2]
    }
    if fd.write(to_write.as_ref()).unwrap() < 0 {
        format!("[sysfs_gpio_direction] error !\r\n");
        exit(-1)
    }
}

pub fn sysfs_gpio_read(pin: i32) -> i32{
    let path = Vec::with_capacity(DIR_MAX_SIZ).as_mut_slice();
    match std::write(&mut path[..DIR_MAX_SIZ], format_args!("/sys/class/gpio/gpio{}/value", pin)) {
        Ok(len) => len,
        Err(_) => Err("[sysfs_gpio_read], Error writing to buffer"),
    }.unwrap();
    let mut fd = File::open(path).expect(format!("[sysfs_gpio_read], Set sysfs_gpio_read failed: Pin {}", pin).as_str());
    let mut buf = Vec::with_capacity(3).as_mut_slice();
    fd.read(buf).expect("[sysfs_gpio_read], error");
    let num = unsafe {
        let ptr = slice.as_mut_ptr() as *mut i32;
        ptr.read_unaligned()
    };
    return num
}

pub fn sysfs_gpio_write(pin: i32, value: i32) {
    let s_values_str = "01";
    let path = Vec::with_capacity(DIR_MAX_SIZ).as_mut_slice();
    match std::write(&mut path[..DIR_MAX_SIZ], format_args!("/sys/class/gpio/gpio{}/value", pin)) {
        Ok(len) => len,
        Err(_) => Err("[sysfs_gpio_write], Error writing to buffer"),
    }.unwrap();
    let mut fd = File::open(path).expect(format!("[sysfs_gpio_write], Set sysfs_gpio_read failed: Pin {}", pin).as_str());
    let mut to_write = "01";
    if value == SYSFS_GPIO_LOW {
        to_write = "11"
    }
    if fd.write(to_write.as_ref()).unwrap() < 0 {
        format!("[sysfs_gpio_write] error !\r\n");
        exit(-1)
    }
}