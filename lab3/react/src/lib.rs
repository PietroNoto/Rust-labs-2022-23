use std::collections::{HashMap, HashSet};

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
/// 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(usize);


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId 
{
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError 
{
    NonexistentCell,
    NonexistentCallback,
}


pub struct ComputeCell<'a, T>
{
    id: ComputeCellId,
    val: Option<T>,
    deps: Vec<(CellId, Option<T>)>,
    fun: Box<dyn Fn(&[T]) -> T + 'a>,
    callbacks: HashMap<CallbackId, Box<dyn FnMut(T) + 'a>>
}


pub struct InputCell<T>
{
    id: InputCellId,
    val: Option<T>
}


pub struct Reactor<'a, T> 
{
    ic: HashMap<InputCellId, InputCell<T>>,
    cc: HashMap<ComputeCellId, ComputeCell<'a, T>>,
    inv_deps: HashMap<CellId, HashSet<ComputeCellId>>,
    cbcells: HashMap<CallbackId, ComputeCellId>
}


impl<'a, T: Copy + PartialEq> ComputeCell<'a, T>
{
    pub fn new<F: 'a + Fn(&[T]) -> T>(
        _id: ComputeCellId,
        _dependencies: Vec<(CellId, Option<T>)>, 
        _compute_func: F,) -> Self
    {
        Self 
        { 
            id: _id, 
            val: None, 
            deps: _dependencies, 
            fun: Box::new(_compute_func), 
            callbacks: HashMap::new()
        }
    }


    pub fn compute(&mut self)
    {
        let values: Vec<T> = self.deps.iter()
            .map(|(k, v)| (*v).unwrap())
            .collect::<Vec<T>>();

        let new_val = (self.fun)(&values);
        
        if self.val.is_some_and(|old_val| old_val != new_val)
        {
            for cb in self.callbacks.values_mut()
            {
                cb(new_val);
            }
        }
        self.val = Some(new_val);
    }


    pub fn update_dep(&mut self, _id: CellId, new_value: T)
    {
        for i  in 0..self.deps.len()
        {
            if self.deps[i].0 == _id
            {
                self.deps[i].1 = Some(new_value);
                return;
            }
        }
    }
}


// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> 
{
    pub fn new() -> Self 
    {
        Self { ic: HashMap::new(), cc: HashMap::new(), inv_deps: HashMap::new(), cbcells: HashMap::new() }
    }


    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, _initial: T) -> InputCellId 
    {
        let id = self.ic.len();
        let c = InputCell {id: InputCellId(id), val: Some(_initial)};
        self.ic.insert(InputCellId(id), c);
        InputCellId(id)
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
    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(
        &mut self,
        _dependencies: &[CellId],
        _compute_func: F,
    ) -> Result<ComputeCellId, CellId> 
    {
        let id = ComputeCellId(self.cc.len());
        let mut cell_val: Vec<(CellId, Option<T>)> = Vec::new();
        
        for dep in _dependencies
        {
            match dep
            {
                CellId::Input(iid) => 
                {
                    if !self.ic.contains_key(iid)
                    {
                        return Err(*dep);
                    }
                },
                CellId::Compute(cid) => 
                {
                    if !self.cc.contains_key(cid)
                    {
                        return Err(*dep);
                    }
                }
            }
            cell_val.push((*dep, self.value(*dep)));

            self.inv_deps.entry(*dep)
                .or_insert(HashSet::new())
                .insert(id); 
        }
        
        let mut c = ComputeCell::new(id, cell_val, _compute_func);
        c.compute();
        self.cc.insert(id, c);

        Ok(id)
    }

    
    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> 
    {
        match id
        {
            CellId::Input(iid) => 
            {
                match self.ic.get(&iid)
                {
                    Some(c) => c.val,
                    None => None
                }
            },
            CellId::Compute(cid) =>
            {
                match self.cc.get(&cid)
                {
                    Some(c) => c.val,
                    None => None
                }
            }
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, _id: InputCellId, _new_value: T) -> bool 
    {
        match self.ic.get_mut(&_id)
        {
            Some(ic) => 
            {
                ic.val = Some(_new_value);
                let mut tree_deps: Vec<ComputeCellId> = Vec::new();
                for ccid in self.inv_deps.get(&CellId::Input(_id)).unwrap()
                {
                    tree_deps.push(*ccid);
                }
                while let Some(ccid) = tree_deps.pop()
                {
                    self.cc
                        .entry(ccid)
                        .and_modify(|cell| 
                        {
                            cell.update_dep(CellId::Input(_id), _new_value);
                            cell.compute();
                        });
                }
                true
            },
            None => false
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
    pub fn add_callback<F: 'a + FnMut(T)>(
        &mut self,
        _id: ComputeCellId,
        _callback: F,
    ) -> Option<CallbackId> 
    {
        match self.cc.get_mut(&_id)
        {
            Some(cc) => 
            {
                let k = CallbackId(cc.callbacks.len());
                cc.callbacks.insert(k, Box::new(_callback));
                self.cbcells.insert(k, _id);

                Some(k)
            },
            None => None
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> 
    {
        if self.cc.get(&cell).is_none()
        {
            return Err(RemoveCallbackError::NonexistentCell);
        }
        if self.cbcells.get(&callback).is_none()
        {
            return Err(RemoveCallbackError::NonexistentCallback);
        }

        self.cc.get_mut(&cell).unwrap().callbacks.remove(&callback);
        self.cbcells.remove(&callback);
        Ok(())
    }
}
