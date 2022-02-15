// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics


// * Use an enum for the box color
enum ShippingBoxColor {
    Brown,
    White
}


impl ShippingBoxColor {
    fn properties (&self) {
        match self {
            ShippingBoxColor::Brown => println!("Shipping box color: brown"),
            ShippingBoxColor::White => println!("Shipping box color: white")
        }
    }
}


// * Use a struct to encapsulate the box dimensions
struct ShippingBoxDimensions {
    width: f64,
    height: f64,
    depth: f64
}

impl ShippingBoxDimensions {
    fn properties (&self) {
        println!("Shipping box width: {:?}", self.width);
        println!("Shipping box height: {:?}", self.height);
        println!("Shipping box depth: {:?}", self.depth);
    }
}


// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    color: ShippingBoxColor,
    dimensions: ShippingBoxDimensions,
    weight: f64,
}


impl ShippingBox {
    // * Implement functionality on the box struct to create a new box
    fn new (
        weight: f64,
        color: ShippingBoxColor,
        dimensions: ShippingBoxDimensions
    ) -> Self {
        Self {
            weight,
            color,
            dimensions
        }
    }

    // * Implement functionality on the box struct to print the characteristics
    fn properties (&self) {
        println!("Printing shipping box properties...");
        self.color.properties();
        self.dimensions.properties();
        println!("Shipping box weight: {:?}", self.weight);
    }
}


fn main() {
    let normal_shipping_box_dimensions = ShippingBoxDimensions {
        width: 5.0,
        height: 5.0,
        depth: 3.0
    };
    let normal_shipping_box = ShippingBox::new(
        7.0,
        ShippingBoxColor::Brown,
        normal_shipping_box_dimensions
    );
    normal_shipping_box.properties();

    let custom_shipping_box_dimensions = ShippingBoxDimensions {
        width: 50.0,
        height: 70.0,
        depth: 30.0
    };
    let custom_shipping_box = ShippingBox::new(
        33.4,
        ShippingBoxColor::White,
        custom_shipping_box_dimensions
    );
    custom_shipping_box.properties();
}
