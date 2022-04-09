use serde::{Serialize, Deserialize};
use libusb::{
    Device, 
    ConfigDescriptor, DeviceHandle
};


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    config_num: u8,
    max_power: u16,
    internal_powered: bool,
    remote_wakeup: bool,
    config_str: String
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.config_num         == other.config_num &&
        self.max_power          == other.max_power &&
        self.internal_powered   == other.internal_powered &&
        self.remote_wakeup      == other.remote_wakeup &&
        self.config_str         == other.config_str
    }
}


impl Config {
    fn new(desc: ConfigDescriptor, handle: &DeviceHandle) -> Self {
        let timeout = std::time::Duration::from_millis(200);
        let lang = handle.read_languages(timeout).unwrap()[0];
        
        let config_str = match handle.read_configuration_string(lang, &desc, timeout) {
            Ok(a) => a,
            Err(_) => "No config description".to_string()
        };
        Config { 
            config_num: desc.number(), 
            max_power: desc.max_power(), 
            internal_powered: desc.self_powered(), 
            remote_wakeup: desc.remote_wakeup(),
            config_str
        }
    }
}



/// structure holding all of the relevant data for a device
#[derive(Serialize, Deserialize, Debug)]
pub struct Dev {
    product: String,
    manufacturer: String,
    serial_num: String,
    
    bus_number: u8,
    address: u8,
    
    d_version_main: u8,
    d_version_minor: u8,
    d_version_sub: u8, 
    
    d_class: u8,
    d_subclass: u8,
    
    d_proto: u8,
    d_vendor_id: u16,
    d_product_id: u16,
    
    configs: Vec<Config>
}


/// make sure we can compare devices to eachother
impl PartialEq for Dev {
    fn eq(&self, other: &Self) -> bool {
        let mut same = true;
        // make sure all configs are the same
        for conf in 0..self.configs.len() {
            if self.configs[conf] != other.configs[conf] {
                same = false;
            }
        }
        let same = same;

        same &&
        self.product            == other.product &&
        self.manufacturer       == other.manufacturer &&
        self.serial_num         == other.serial_num &&
        self.bus_number         == other.bus_number && 
        self.address            == other.address &&
        self.d_version_main     == other.d_version_main &&
        self.d_version_minor    == other.d_version_minor &&
        self.d_version_sub      == other.d_version_sub &&
        self.d_class            == other.d_class &&
        self.d_subclass         == other.d_subclass &&
        self.d_proto            == other.d_proto &&
        self.d_vendor_id        == other.d_vendor_id &&
        self.d_product_id       == other.d_product_id
    }
}



/// make sure we can pretty print a device
impl std::fmt::Display for Dev {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "\tProduct={}\n\tManufacturer={}\n\tSerial Number={}\n\tBus Number={}\n\tAddress={}\n\tUSB Version={}.{}.{}\n\tUSB Class={}\n\tSubclass={}\n\tProtocol={}\n\tVendor Id={}\n\tProduct ID={}",
            self.product,
            self.manufacturer,
            self.serial_num,
            self.bus_number, 
            self.address, 
            self.d_version_main, self.d_version_minor, self.d_version_sub,
            self.d_class, 
            self.d_subclass, 
            self.d_proto,
            self.d_vendor_id,
            self.d_product_id
        )
    }
}


impl Dev {
    /// creates a new device from a DeviceInfo reference
    pub fn new(device: Device) -> Self {
        // handle strings that can exist or not
        let desc = device.device_descriptor().unwrap();
        let mut configs: Vec<Config> = Vec::new();
        let handle = device.open().unwrap();
        

        // save all of the configs
        for index in 0..desc.num_configurations() {
            let cfg_tmp = device.config_descriptor(index).unwrap();
            configs.push(Config::new(cfg_tmp, &handle));
        }

        // create variables we need in a sec 
        let v = desc.device_version();
        let timeout = std::time::Duration::from_millis(200);
        
        let lang = handle.read_languages(timeout).unwrap()[0];
        let manufacturer = match handle.read_manufacturer_string(lang, &desc, timeout) {
            Ok(a) => a,
            Err(_) => "No Manufacturer Information Available".to_string()
        };
        let product = match handle.read_product_string(lang, &desc, timeout) {
            Ok(a) => a,
            Err(_) => "No Product Information Available".to_string()
        };
        let serial_num = match handle.read_serial_number_string(lang, &desc, timeout) {
            Ok(a) => a,
            Err(_) => "No Serial Number Available".to_string()
        };
        
        // create entry
        Dev {
            product,
            manufacturer,
            serial_num,
        
            bus_number:         device.bus_number(),
            address:            device.address(),

            d_version_main:     v.major(),
            d_version_minor:    v.minor(),
            d_version_sub:      v.sub_minor(),
            
            d_class:            desc.class_code(),
            d_subclass:         desc.sub_class_code(),
            
            d_proto:            desc.protocol_code(),  
            d_vendor_id:        desc.vendor_id(),
            d_product_id:       desc.product_id(),
            
            configs
        }
    }
}

