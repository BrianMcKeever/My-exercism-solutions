use std::collections::HashMap;

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputCellID(usize);
/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputeCellID(usize);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct Compute<'a, T> {
    dependencies: Vec<CellID>,
    compute_func: Box<dyn Fn(&[T]) -> T + 'a>,
    value: T,
    old_value: T,
    callbacks: HashMap<usize, Box<dyn FnMut(T) -> () + 'a>>,
    callback_id: usize,
    dependants: Vec<ComputeCellID>,
}

struct Input<T> {
    value: T,
    dependants: Vec<ComputeCellID>,
}

pub struct Reactor<'a, T> {
    input_cells: Vec<Input<T>>,
    compute_cells: Vec<Compute<'a, T>>,
}

impl<T: Copy + PartialEq> Input<T> {
    fn new(value: T)-> Self {
        Input { value: value, dependants: Vec::new()}
    }
    fn add_dependant(&mut self, compute_cell: ComputeCellID) {
        self.dependants.push(compute_cell);
    }
}

impl<'a, T: Copy + PartialEq> Compute<'a, T> {
    fn new<F: Fn(&[T]) -> T + 'a>(value: T, dependencies: &[CellID], compute_func: F) -> Self {
        Compute {
            dependencies: dependencies.to_vec(),
            value: value,
            compute_func: Box::new(compute_func),
            callbacks: HashMap::new(),
            callback_id: 0,
            old_value: value,
            dependants: Vec::new(),
        }
    }
    fn add_dependant(&mut self, compute_cell: ComputeCellID) {
        self.dependants.push(compute_cell);
    }
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq + std::fmt::Debug> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            input_cells: Vec::new(),
            compute_cells: Vec::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        self.input_cells.push(Input::new(initial));
        InputCellID(self.input_cells.len() - 1)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        let values = self.dependencies_to_values(dependencies)?;
        let value = compute_func(&values);
        let c = Compute::new(value, dependencies, compute_func);
        let id = ComputeCellID(self.compute_cells.len());
        for cell_id in dependencies {
            match cell_id {
                CellID::Input(InputCellID(i))     => self.  input_cells.get_mut(*i).as_mut().map(|x| x.add_dependant(id)),
                CellID::Compute(ComputeCellID(i)) => self.compute_cells.get_mut(*i).as_mut().map(|x| x.add_dependant(id)),
            };
        }
        self.compute_cells.push(c);
        Ok(id)
    }

    fn dependencies_to_values(&self, dependencies: &[CellID]) -> Result<Vec<T>, CellID> {
        dependencies.iter().copied().map(|x| self.value(x).ok_or(x)).collect()
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(InputCellID(i)) => self.input_cells.get(i).map(|x| x.value),
            CellID::Compute(ComputeCellID(i)) => self.compute_cells.get(i).map(|x| x.value),
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, InputCellID(id): InputCellID, new_value: T) -> bool {
        let input = self.input_cells.get_mut(id);
        if input.is_none() {
            return false;
        }
        let input = input.unwrap();
        input.value = new_value;
        for compute_id in input.dependants.clone().iter() {
            self.update_compute(*compute_id);
        }
        for compute in self.compute_cells.iter_mut() {
            if compute.value != compute.old_value {
                compute.old_value = compute.value;
                for key in compute.callbacks.keys().cloned().collect::<Vec<_>>() {
                    let f = compute.callbacks.get_mut(&key).unwrap();
                    (f)(compute.value);
                }
            }
        }
        true
    }

    fn update_compute(&mut self, ComputeCellID(id): ComputeCellID) {
        let compute = self.compute_cells.get(id);
        if compute.is_none() {
            return;
        }
        let compute = compute.unwrap();
        let old_value = compute.value;
        let values = self.dependencies_to_values(&compute.dependencies).unwrap();
        let compute = &mut self.compute_cells[id];
        compute.value = (compute.compute_func)(&values);
        if compute.value != old_value {
            for compute_id in compute.dependants.clone().iter() {
                self.update_compute(*compute_id);
            }
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) -> () + 'a>(
        &mut self,
        ComputeCellID(id): ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        let mut compute = self.compute_cells.get_mut(id)?;
        compute.callbacks.insert(compute.callback_id, Box::new(callback));
        let result = CallbackID(compute.callback_id);
        compute.callback_id += 1;
        Some(result)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        ComputeCellID(com_id): ComputeCellID,
        CallbackID(call_id): CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        let compute = self.compute_cells.get_mut(com_id);
        if compute.is_none(){
            return Err(RemoveCallbackError::NonexistentCell);
        }
        if compute.unwrap().callbacks.remove(&call_id).is_none() {
            return Err(RemoveCallbackError::NonexistentCallback);
        }
        Ok(())
    }
}
