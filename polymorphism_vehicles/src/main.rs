struct Sedan;
impl LandCapable for Sedan {
    fn drive(&self) {
        println!("Sedan is driving");
    }
}

struct SUV;
impl LandCapable for SUV {
    fn drive(&self) {
        println!("SUV is driving");
    }
}

trait LandCapable {
    fn drive(&self) {
        println!("Default drive");
    }
}

trait WaterCapable {
    fn float(&self) {
        println!("Default float");
    }
}

trait Amphibious {
    fn all_terrain(&self) {
        println!("All terrain");
    }
}
impl<T: LandCapable + WaterCapable> Amphibious for T {}

struct Hovercraft;
// impl Amphibious for Hovercraft {}
impl LandCapable for Hovercraft {}
impl WaterCapable for Hovercraft {}

fn road_trip(vehicle: &impl LandCapable) {
    vehicle.drive();
}

fn cross_river(vehicle: &impl WaterCapable) {
    vehicle.float();
}

fn cross_jungle(vehicle: &impl Amphibious) {
    vehicle.all_terrain();
}

fn main() {
    let sedan = Sedan;

    let suv = SUV;

    road_trip(&sedan);
    road_trip(&suv);

    let overcraft = Hovercraft;
    cross_river(&overcraft);

    cross_jungle(&overcraft);
}
