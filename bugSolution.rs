Several solutions exist to fix multiple mutable borrows:

**1. Cloning:** If the variable's data is cheap to copy, you can just clone it before making multiple mutable borrows.

```rust
fn main() {
    let mut x = 5;
    let mut y = x.clone(); 
    let mut z = x.clone();

    y += 1;
    z += 1;
    println!("x: {}, y: {}, z: {}", x, y, z);
}
```

**2. Single Mutable Reference:**  Only borrow the variable once and perform all modifications within that scope:

```rust
fn main() {
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
        *y += 1; //This is allowed
    }
    println!("x: {}", x);
}
```

**3. Using a struct and interior mutability (for more complex scenarios):**

```rust
use std::sync::Mutex;

fn main() {
    let x = Mutex::new(5);
    let mut y = x.lock().unwrap();
    let mut z = x.lock().unwrap();
    *y +=1;    *z += 1;//This is allowed in this scenario

}
```
Choose the solution that best fits the context of your code and data.  Cloning is easy if the copy is inexpensive.  A single mutable reference is best when possible for simplicity and efficiency.  Interior mutability using Mutex or other similar types are for more complex concurrent scenarios.