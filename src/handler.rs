use crate::DimensionsHelper::DimensionsHelper;
use crate::structs::{Container, ContainerSpec, ContainerTypeVolume, ContainingProduct, OrderRequest, ShipmentRecord, TotalVolume};
use crate::structs::Product;

pub struct ProductsHandler {}

impl ProductsHandler {
    pub fn getProductVolume(&self, product: &Product) -> i32 {
        DimensionsHelper::getDimensionsVolume(&product.dimensions)
    }

    pub fn getProductVolumePerOrderedQuantity(&self, product: &Product) -> i32 {
        self.getProductVolume(product) * product.orderedQuantity
    }
}

pub struct ContainersHandler {
    pub containerSpecs: Vec<ContainerSpec>,
}

impl ContainersHandler {
    pub fn getContainerVolume(&self, containerSpecs: &ContainerSpec) -> i32 {
        DimensionsHelper::getDimensionsVolume(&containerSpecs.dimensions)
    }

    pub fn getContainerTypeVolume(&self, containerType: &str) -> i32 {
        let containerSpec: Vec<&ContainerSpec> = self.containerSpecs.iter()
            .filter(|containerSpec| containerSpec.containerType == containerType)
            .collect();

        self.getContainerVolume(containerSpec.get(0).unwrap())
    }

    pub fn getContainerTypesVolume(&self) -> Vec<ContainerTypeVolume> {
        self.containerSpecs.iter()
            .map(|containerSpecs| ContainerTypeVolume {
                containerType: containerSpecs.containerType.clone(),
                volume: self.getContainerVolume(&containerSpecs),
            })
            .collect()
    }
}

pub struct OrderHandler {
    pub containersHandler: ContainersHandler,
    pub productsHandler: ProductsHandler,
}

impl OrderHandler {
    pub fn packOrder(&self, orderRequest: OrderRequest) -> ShipmentRecord {
        self.checkOrderExecutable(&orderRequest);
        let containers: Vec<Container> = self.getContainers(&orderRequest);

        ShipmentRecord {
            orderId: orderRequest.id.to_string(),
            totalVolume: TotalVolume { unit: "cubic centimeter".to_string(), value: self.getTotalVolume(&containers) },
            containers,
        }
    }

    fn checkOrderExecutable(&self, orderRequest: &OrderRequest) {
        let containerTypesVolume = self.containersHandler.getContainerTypesVolume();

        for product in &orderRequest.products {
            let productVolume = self.productsHandler.getProductVolume(&product);

            let mut timesProductVolumeGreater = 0;
            for containerTypeVolume in &containerTypesVolume {
                if productVolume > containerTypeVolume.volume {
                    timesProductVolumeGreater += 1
                }
            }

            if timesProductVolumeGreater == containerTypesVolume.len() {
                panic!("Order can't be executed, since one of it's product(s) {} volume exceeds available containers volume.", product.id);
            }
        }
    }

    fn getContainers(&self, orderRequest: &OrderRequest) -> Vec<Container> {
        let mut containers: Vec<Container> = vec![];
        let availableContainers = &self.containersHandler.containerSpecs;

        for product in &orderRequest.products {
            let mut quantityAdded = 0;
            let i = 0;
            while i < availableContainers.len() {
                let containerSpec = availableContainers.get(i).unwrap();
                let mut containingProducts: Vec<ContainingProduct> = vec![];

                if self.canStoreProduct(containerSpec, &product) {
                    if self.canStoreProductPerOrderedQuantity(containerSpec, &product) {
                        containers.push(Container {
                            containerType: containerSpec.containerType.clone(),
                            containingProducts: self.addToContainingProducts(&mut containingProducts, &product.id, product.orderedQuantity),
                        });

                        quantityAdded += &product.orderedQuantity;
                        break; // no need to check in next container
                    }
                } else {
                    let howManyCanBeStored = self.howManyCanBeStored(&containerSpec, &product);
                    containers.push(
                        Container {
                            containerType: containerSpec.containerType.clone(),
                            containingProducts: self.addToContainingProducts(&mut containingProducts, &product.id, howManyCanBeStored),
                        }
                    );
                    quantityAdded += howManyCanBeStored
                }
            }

            let diff = product.orderedQuantity - quantityAdded;
            if diff > 0 {
                // same container could be used multiple times
                let mut i = 0;
                while i < diff {
                    containers.push(Container {
                        containerType: containers.clone().get(0).unwrap().containerType.to_owned(),
                        containingProducts: containers.clone().get(0).unwrap().containingProducts.to_owned(),
                    });
                    i += 1;
                }
            }
        }

        containers
    }

    fn canStoreProduct(&self, containerSpec: &ContainerSpec, product: &Product) -> bool {
        return self.containersHandler.getContainerVolume(&containerSpec) >= self.productsHandler.getProductVolume(product);
    }

    fn canStoreProductPerOrderedQuantity(&self, containerSpec: &ContainerSpec, product: &Product) -> bool {
        self.containersHandler.getContainerVolume(containerSpec) >= self.productsHandler.getProductVolumePerOrderedQuantity(product)
    }

    fn addToContainingProducts(&self, containingProducts: &mut Vec<ContainingProduct>, id: &String, quantity: i32) -> Vec<ContainingProduct> {
        containingProducts.push(ContainingProduct { id: id.to_string(), quantity });
        return containingProducts.to_owned();
    }

    fn howManyCanBeStored(&self, containerSpec: &ContainerSpec, product: &Product) -> i32 {
        let containerVolume = self.containersHandler.getContainerVolume(&containerSpec);
        let productVolume = self.productsHandler.getProductVolume(&product);
        let mut adjustableVolume = productVolume;

        let mut quantity = 0;
        while adjustableVolume <= containerVolume {
            adjustableVolume += productVolume;
            quantity += 1
        }

        return quantity;
    }

    fn getTotalVolume(&self, containers: &Vec<Container>) -> i32 {
        let mut totalVolume = 0;
        for container in containers.iter() {
            totalVolume += self.containersHandler.getContainerTypeVolume(&container.containerType.as_str())
        }

        totalVolume
    }
}
