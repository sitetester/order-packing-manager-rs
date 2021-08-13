pub struct ContainerSpec {
    pub containerType: String,
    pub dimensions: Dimensions,
}

pub struct Dimensions {
    pub unit: String,
    pub length: i32,
    pub width: i32,
    pub height: i32,
}

pub struct OrderRequest {
    pub id: String,
    pub products: Vec<Product>,
}

pub struct Product {
    pub id: String,
    pub name: String,
    pub orderedQuantity: i32,
    pub unitPrice: f32,
    pub dimensions: Dimensions,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TotalVolume {
    pub unit: String,
    pub value: i32,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ContainingProduct {
    pub id: String,
    pub quantity: i32,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Container {
    pub containerType: String,
    pub containingProducts: Vec<ContainingProduct>,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ShipmentRecord {
    pub orderId: String,
    pub totalVolume: TotalVolume,
    pub containers: Vec<Container>,
}

pub struct ContainerTypeVolume {
    pub containerType: String,
    pub volume: i32,
}