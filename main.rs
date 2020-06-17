fn main() {
  //TODO
}

// STAC stands for Semisupervised Ternary Agglomerative Clustering
// It is usually Agglomerative Clustering where the exit condition is merging labels
// Ternary state allows failure of one link to "try again" eta times
// It takes only eta as a hyperparameter (not sure if labels count)

trait STACModelConstructor {
  // The constructor itself
  // Takes the data to initialize with, a vector of labeled boolean space points
  // Returns the new STAC object
  fn new(given_data: Vec<LabelBoolPoint>) -> STAC
}

trait STACModel {
  // Trains the STAC model
  // Training must be a method that takes the eta hyperparameter
  // Training must return the Some(Trained) or None if it fails
  // Its work is stored in the self.result vector of cluster IDs
  fn train(&mut self, eta: u32) -> option<Trained>,
  // Checks if a and b are in the same cluster
  // Takes a and b (points in a boolean space) as Vectors of booleans
  // Returns Some(ConnectEnum) if self.result.job is TaskState::done, otherwise None
  // Its work is stored in the return, and must not mutate self
  // Note that a and b are Vec<bool> not LabelBoolPoint
  // This is because we do not use label information
  fn same_cluster(&self, a: Vec<bool>, b: Vec<bool>) -> option<ConnectEnum>,
  // This is used to update the STAC.data value
  // Do not do this directly
  // This will automatically await the STAC.trained TaskState
  // It will also set the STAC.trained TaskState
  // Note that it calls STAC::new, and then passes on the fields
  fn update_data(&mut self, new_data: Vec<LabelBoolPoint>)
}

struct STAC {
  // Vector of points in a boolean space, some labeled, use LabelBoolPoint struct
  data: Vec<LabelBoolPoint>,
  // Job and Vector of cluster IDs
  result: VecJob<option<u32>>,
  // Internal, attempted to link, failed, but ternary allowed alternate link
  attempted_failed: Vec<NewLink<Vec<bool>>>
}

// Constructor impl block
impl STACModelConstructor for STAC {
  // The constructor
  fn new(given_data: Vec<LabelBoolPoint>) -> self {
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
    // Check that self.result.job is TaskState::ready
    let result_return
    if self.result.job != TaskState::ready {
      return None // Return None
    }
    // Set that the train task is pending
    self.result.job = TaskState::pending;
    // Iterate until deemed complete by STAC::training_iteration
    while self.result.job == TaskState::pending {
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
    };
    // Convert the option<bool> to option<ConnectEnum>
    match same_cluster_boolean {
      Some(true) => Some(ConnectEnum::linked), // Same cluster
      Some(false) => Some(ConnectEnum::seperate), // Different cluster
      None => None // Something went wrong
    }
  }
  
  // The update_data function, see STACModel
  fn update_data(&mut self, newdata: Vec<LabelBoolPoint>) {
    // Await the self.result.job TaskState to be not pending
    while self.result.job == TaskState::pending {};
    // Set the self.result.job TaskState to be pending
    self.result.job = TaskState::pending;
    // Generate a new STAC object with the desired fields
    let new_STAC_object = STAC::new(newdata);
    // Update the data field
    self.data = new_STAC_object.data;
    // Update the result field
    self.result = new_STAC_object.result;
    // Set self.result.job TaskState to ready
    self.result.job = TaskState::ready;
    // Implicitly return unit
  }
}

// Internal training methods, see STAC::train
impl STAC {
  // Training iteration, called by STAC::train, an impl of STACModel trait
  fn training_iteration(&mut self, eta: u32) {
    // TODO
  }
  fn attempt_connect_closest(&mut self, eta: u32) {
    // Set the current job state to pending
    self.result.job = TaskState::pending;
    // Map each datapoint to the distance to the closest other in boolean space
    let closest_vec = self.data
      .iter() // Convert to iterable
      .map(|&x| {
        // Returns a tuple of (point: Vec<bool>, distance: u64)
        Distance::closest_boolean_space(x.clone().to_vec(), self.data)
      })
      .collect::<Vec<(Vec<bool>, u64)>>(); // Iterator<_> -> Vec<_>
    // Make a new vector of just the distance (u64) of the tuple
    let dists = closest_vec
      .iter() // Convert to iterable ( -> Iterator<u64> )
      .map(|&x| {
        x.1 // Access the second index (the u64) of the tuple (Vec<bool>, u64)
      })
      .collect::<Vec<u64>>(); // Iterator<u64> -> Vec<u64>, used turbofish
    let min_dist = dists
      .iter() // -> Iterator<u64>
      .fold(|a, x| { // u64 implements the Copy trait, so no need for reference
        if x < a {x} else {a} // Choose the minimum of either u64
      });
    // Find the index of the closest pair, i.e. the pair with minimum distance
    let closest_index = dists
      .iter() // -> Iterator<u64>
      .position(|x| { // No reference, u64 implements the Copy trait
        x == min_dist // Find the position of the element that is min_dist
      }); // -> usize, so we can use as an index without a cast or usize::from
    // Get the first point
    let datapoint_a = self.data[closest_index];
    // And then the other datapoint, as the Vec<bool> entry of the right tuple
    let datapoint_b = closest_vec[closest_index].0;
  }
}

// Represent a potentially labeled point in boolean space
struct LabelBoolPoint {
  point: Vec<bool>, // The point itself
  label: option<LabelEnum> // Some(LabelEnum) if labels, otherwise None
}

// Represent a job associated with a generic vector using TaskState
struct VecJob<T> {
  vector: Vec<T>,
  job: TaskState
}

// Represent a generic link replaced with another
struct NewLink<T> {
  initial: (T,T),
  resolve: (T,T)
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
