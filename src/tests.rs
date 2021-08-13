#[cfg(test)]
mod tests {
    use crate::handler::{ContainersHandler, OrderHandler, ProductsHandler};
    use crate::structs::{Container, ContainerSpec, ContainingProduct, Dimensions, OrderRequest, Product, ShipmentRecord, TotalVolume};

    fn getContainerSpecs() -> Vec<ContainerSpec> {
        vec![
            ContainerSpec {
                containerType: "Cardboard A".to_string(),
                dimensions: Dimensions {
                    unit: "centimeter".to_string(),
                    length: 30,
                    width: 30,
                    height: 30,
                },
            },
        ]
    }

    #[test]
    fn given_a_small_order_pack_it_into_a_single_container() {
        let orderRequest = OrderRequest {
            id: String::from("ORDER-001"),
            products: vec![
                Product {
                    id: "PRODUCT-001".to_string(),
                    name: "GOOD FORTUNE COOKIES".to_string(),
                    orderedQuantity: 9,
                    unitPrice: 13.4,
                    dimensions: Dimensions {
                        unit: "centimeter".to_string(),
                        length: 10,
                        width: 10,
                        height: 30,
                    },
                }
            ],
        };

        let expectedShipmentRecord = ShipmentRecord {
            orderId: "ORDER-001".to_string(),
            totalVolume: TotalVolume { unit: "cubic centimeter".to_string(), value: 27000 },
            containers: vec![Container {
                containerType: "Cardboard A".to_string(),
                containingProducts: vec![ContainingProduct {
                    id: "PRODUCT-001".to_string(),
                    quantity: 9,
                }],
            }],
        };

        let orderHandler = OrderHandler {
            containersHandler: ContainersHandler { containerSpecs: getContainerSpecs() },
            productsHandler: ProductsHandler {},
        };


        assert_eq!(orderHandler.packOrder(orderRequest), expectedShipmentRecord);
    }
}