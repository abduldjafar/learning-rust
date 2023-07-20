use systemstat::{System, Platform, saturating_sub_bytes};


pub struct SystemStats{
    system: System,
}

impl SystemStats{
    pub fn new() -> Self{
        let system = System::new();

        return SystemStats{
            system
        };
    } 

    pub fn monitoring_mounted_path( &self){
        match self.system.mounts() {
            Ok(mounts) => {
                println!("\nPath Mounted Monitoring:");
                for mount in mounts.iter() {
                    println!("================================================");
                    println!("");
                    println!("================================================");
                    println!("mounted from      : {}",mount.fs_mounted_from);
                    println!("fs type           : {}",mount.fs_type);
                    println!("mounted on        : {}",mount.fs_mounted_on);
                    println!("total size        : {}",mount.total);
                    println!("precentage usage  : {}%",format!("{:.2}", (100.0*((mount.total.as_u64() - mount.avail.as_u64()) as f32/mount.total.as_u64() as f32))));                
                }
            }
            Err(x) => println!("\nMounts: error: {}", x)
        }
    }

    pub fn monitoring_memory_resources(&self){
        match self.system.memory() {
            Ok(mem) => {
                println!("\nMemory Monitoring:");
                println!("================================================");
                println!("");
                println!("================================================");
                println!("usage: {}",saturating_sub_bytes(mem.total, mem.free));
                println!("total: {}",mem.total);
            },
            Err(x) => println!("\nMemory: error: {}", x)
        }
    }
}
