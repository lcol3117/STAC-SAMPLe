fn main() {
  println!("Hello World!");
}

pub trait STACModel {
  // Trains the STAC model
  // Training must be a method that takes the eta hyperparameter
  // Training must return the Some(Trained) or None if it fails
  // Its work is stored in the self.result vector of cluster IDs
  fn train(&mut self, eta: u32) -> option<Trained>,
  // Checks if a and b are in the same cluster
  // Takes a and b (points in a boolean space) as Vectors of booleans
  // Returns Some(ConnectEnum) if self.trained is DoneEnum::done, otherwise None
  // Its work is stored in the return, and must not mutate self
  // Note that a and b are Vec<bool> not BooleanSpacePoint
  // This is because we do not use label information
  fn same_cluster(&self, a: Vec<bool>, b: Vec<bool>) -> option<ConnectEnum>,
  // This is used to update the STAC.data value
  // Do not do this directly
  // This will automatically await the STAC.trained TaskState
  // It will also set the STAC.trained TaskState
  // Note that it calls STAC::new, and then passes on the fields
  fn update_data(&mut self, new_data: Vec<BooleanSpacePoint>)
}

pub struct STAC {
  // Vector of points in a boolean space, use BooleanSpacePoint struct
  data: Vec<BooleanSpacePoint>,
  // Vector of cluster IDs
  result: Vec<u32>,
  // Is the training done, ready, or pending
  trained: TaskState
}

// Constructor impl block
impl STAC {
  // The constructor
  fn new(given_data: Vec<BooleanSpacePoint>) -> self {
    // Generate the cluster IDs s.t. all points are seperate
    let intial_result = (0_u32..(data.len() as u32)) // Range<u32>
      .collect::<Vec<u32>>(); // Vec<u32>
    // Build the STAC object
    STAC {
      data: given_data // Use given data
      result: intial_result, // Use generated cluster IDs
      trained: TaskState::ready // Ready to train
    }
  }
}

// Trait functions, see STACModel
impl STACModel for STAC {
  // The train function, see STACModel
  fn train(&mut self, eta: u32) -> option<Trained> {
    // Check that self.trained is TaskState::ready
    let result_return
    if self.trained != TaskState::ready {
      return None // Return None
    }
    // Set that the train task is pending
    self.trained = TaskState::pending;
    // Iterate until deemed complete by STAC::training_iteration
    while self.trained == TaskState::pending {
      // Need to pass on eta, it is not a property
      self.training_iteration(eta: u32); // Call STAC::training_iteration
    };
    // Return Some of the Trained unit struct, to represent completion
    // Note that the STAC::trained property is set to TaskState::done
    // This is done by the STAC::training_iteration method
    return Some(Trained)
  }
  
  // The same_cluster function, see STACModel
  fn same_cluster(&self, a: Vec<bool>, b: Vec<bool>) -> option<ConnectEnum> {
    // Locate a in self.data, and get the index
    let a_index = self.data
      .iter() // Convert to iterator
      .position(|&x| {x.to_vec() == a}) // Get the position of a
      .unwrap(); // Assert it exists
    // Locate b in self.data, and get the index
    let b_index = self.data
      .iter() // Convert to iterator
      .position(|&x| {x.to_vec() == b}) // Get the position of b
      .unwrap(); // Assert it exists
    // Get the cluster ID of a
    let a_cluster_id = match a_index {
      Some(index) => self.result[index],
      None => None
    };
    // Get the cluster ID of b
    let b_cluster_id = match b_index {
      Some(index) => self.result[index],
      None => None
    };
    // Check the cluster IDs, and see if they are equivalent
    let same_cluster_boolean = match a_cluster_id {
      // If a_cluster_id is not None, check b_cluster_id
      Some(a_cluster_id) => match b_cluster_id {
        Some(b_cluster_id) => Some(a_cluster_id == b_cluster_id),
        None => None // Carry the None
      },
      None => None // Carry the None
    }
    // Convert the option<bool> to option<ConnectEnum>
    match same_cluster_boolean {
      Some(true) => Some(ConnectEnum::linked), // Same cluster
      Some(false) => Some(ConnectEnum::seperate), // Different cluster
      None => None // Something went wrong
    }
  }
}

// Represent a potentially labeled point in boolean space
struct BooleanSpacePoint {
  point: Vec<bool>, // The point itself
  label: option<LabelEnum> // Some(LabelEnum) if labeles, otherwise None
}

// Represent the labels used in SAMPLe
enum LabelEnum {
  malware, // Malicious packages
  accept // Acceptable packages
}

// Implement copy and clone traits for LabelEnum
impl Copy for LabelEnum {}
impl Clone for LabelEnum {
  fn clone(&self) -> self {
    *self // Just return the enum value itself
  }
}

// Represent the state of a task
enum TaskState {
  done, // The task is complete
  ready, // The task has not been started
  pending // The task is currently running
}

// Implement copy and clone traits for TaskState
impl Copy for TaskState {}
impl Clone for TaskState {
  fn clone(&self) -> self {
    *self // Just return the enum value itself
  }
}

// Represent connectivity
enum ConnectEnum {
  linked, // The points are linked, in the same cluster
  seperate // The points are in seperate clusters
}

// Implement copy and clone traits for ConnectEnum
impl Copy for ConnectEnum {}
impl Clone for ConnectEnum {
  fn clone(&self) -> self {
    *self // Just return the enum value itself
  }
}

struct Trained; // Unit struct, used to represent training attempt is complete
