
use std::ops::{Add, Mul, Sub};

use std::thread;
use std::thread::JoinHandle;
use std::sync::{Mutex, Arc};

use rand::Rng;


//definition

#[derive(Clone)]
pub struct Matrix<T> { 
   pub dimensions: (usize, usize),
    pub values: Vec<Vec<T>>,
}

//Matrix creation functions here
impl<T: Copy + Default> Matrix<T> {

    //can only fail if the value vector is not symetrical
    pub fn from_vec(values: &Vec<Vec<T>>) -> Result<Matrix<T>, String> {
        if valid_size(values) == false {
            return Err(String::from("The vector has inconsistent dimensions"));
        }

        let dimensionss = (values.len(), values[0].len()); 
        let val = values.clone();

        Ok(Matrix::<T> {
            dimensions: dimensionss,
            values: val,
        })
    }

    pub fn new(y_size: usize, x_size: usize) -> Matrix<T> {
        let new_dimensions = (y_size, x_size);
        let default_value = T::default();
        let new_values = vec![vec![default_value.clone(); x_size]; y_size];
    
        Matrix::<T> {
            dimensions: new_dimensions,
            values: new_values,
        }
    }

    
}

impl Matrix<f64> {
    pub fn rand_new(y_size: usize, x_size: usize) -> Matrix<f64> {
        let new_dimensions = (y_size, x_size);

        let mut values = vec![vec![0.0; x_size]; y_size];
        
        let mut rng = rand::thread_rng();
        for y in 0..y_size {
            for x in 0..x_size {
                values[y][x] = rng.gen();
            }
        }

        Matrix::<f64> {
            dimensions: new_dimensions,
            values: values,
        }
    }
}


//Multiplication & Hadamard & Transpose Implementation
impl<T:Default + Copy + Send + Sync + 'static + std::ops::Mul + Add<Output = T> + Mul<Output = T>> Matrix<T> {

    //returns matrix don't think it can ever fail
    pub fn transpose(&self) -> Matrix<T> {
        
        let dim_x = &self.dimensions.1;
        

        let new_vec: Arc<Mutex<Vec<Vec<T>>>> = Arc::new(Mutex::new(vec![vec![]; dim_x.clone()])); //creates vector of length to be access by threads

        let values = Arc::new(self.values.clone());
        let mut threads: Vec<JoinHandle<()> > = vec![];

        for i in 0..dim_x.clone() {

            let cloned_values = Arc::clone(&values);
            let cloned_output = Arc::clone(&new_vec);
            let index = Arc::new(i);

            let handle = thread::spawn(move || {
                let x = cloned_values;
                let ind = index;
                

                let mut val: Vec<T> = vec![];

                for y in 0..x.len() {
                    val.push(x[y][*ind]);
                }

                let mut output = cloned_output.lock().unwrap();
                output[*ind] = val;
                
            });

            threads.push(handle);
        }

        
        for thread in threads {
            thread.join().unwrap();
        }
        
        let new_vec = new_vec.lock().unwrap();

        return Matrix::from_vec(&new_vec).unwrap();
    }

    pub fn multiply(&self, mat2: &Matrix<T>) -> Result<Matrix<T>, String> {
        if self.dimensions.1 != mat2.dimensions.0 {
            return Err(String::from("Attempted to multiply 2 matricies that cannot be multiplied due to dimensions"));
        }

        //create 2d vector for our threads to put their values in
        let mut output_values: Arc<Mutex<Vec<Vec<T>>>> = Arc::new(Mutex::new(vec![vec![]; self.dimensions.0]));

        //create references for our threads to access and compute their values
        let lhs = Arc::new(self.clone());
        let rhs = Arc::new(mat2.clone());
        
        let mut threads: Vec<JoinHandle<()> > = vec![];

        //create thread for each row in the output matrix
        for row_index in 0..self.dimensions.0 {
            let cloned_lhs = lhs.clone();
            let cloned_rhs = rhs.clone();

            let output = output_values.clone();

            let thread = thread::spawn(move || {
                //all values needed are now in the thread
                let index = row_index;
                let lhs_ref = cloned_lhs;
                let rhs_ref = cloned_rhs;

                //lhs row
                let row = &lhs_ref.values[index];
                let mut out_row: Vec<T> = vec![];

                //iterate over all rhs columns
                for col in 0..rhs_ref.values[0].len() {
                    let mut c = T::default();
                    for i in 0..row.len() {
                        c = c + (row[i] * rhs_ref.values[i][col]);
                    }
                    out_row.push(c);
                }
                let mut output = output.lock().unwrap();
                output[index] = out_row;

            });


            threads.push(thread);
        }

        for thread in threads {
            thread.join().unwrap();
        }

        let vec = output_values.lock().unwrap();

        let mat = Matrix::from_vec(&vec).unwrap();

        return Ok(mat)
    }

    pub fn hadamard_mult(&self, mat: &Matrix<T>) -> Result<Matrix<T>, String> {
        if self.dimensions != mat.dimensions {
            return(Err("Matrix dimensions are not equal to do a Hadarmard multiplication".to_string()));
        }

        
        let output_mat: Matrix<T> = Matrix::new(self.dimensions.0, self.dimensions.1);
        let output_mat = Arc::new(Mutex::new(output_mat));
        
        let mut threads: Vec<JoinHandle<()>> = vec![];
        for row in 0..self.dimensions.0 {

            let mut row1 = self.values[row].clone();
            let row2 = mat.values[row].clone();

            let output_clone = output_mat.clone();

            let thread = thread::spawn(move || {
                let output_thread_clone = output_clone.clone();

                //hadamard the row
                for i in 0..row1.len() {
                    row1[i] = row1[i] * row2[i];
                }

                let mut output_thread_clone_lock = output_thread_clone.lock().unwrap();

                output_thread_clone_lock.values[row] = row1;

            });

            threads.push(thread);
        }

        for thread in threads {
            thread.join().unwrap();
        }

        let output = output_mat.lock().unwrap();
        Ok(output.clone())

    }
    //multiplies by the transpose of mat2
    pub fn multiply_by_transpose(&self, mat2: &Matrix<T>) -> Result<Matrix<T>, String> {
        if self.dimensions.1 != mat2.dimensions.1 {
            return Err(String::from("Attempted to multiply the transpose of mat 2 that cannot be multiplied due to dimensions"));
        }

        //create 2d vector for our threads to put their values in
        let mut output_values: Arc<Mutex<Vec<Vec<T>>>> = Arc::new(Mutex::new(vec![vec![]; self.dimensions.0]));

        //create references for our threads to access and compute their values
        let lhs = Arc::new(self.clone());
        let rhs = Arc::new(mat2.clone());
        
        let mut threads: Vec<JoinHandle<()> > = vec![];

        //create thread for each row in the output matrix
        for row_index in 0..self.dimensions.0 {
            let cloned_lhs = lhs.clone();
            let cloned_rhs = rhs.clone();

            let output = output_values.clone();

            let thread = thread::spawn(move || {
                //all values needed are now in the thread
                let index = row_index;
                let lhs_ref = cloned_lhs;
                let rhs_ref = cloned_rhs;

                //lhs row
                let row = &lhs_ref.values[index];
                let mut out_row: Vec<T> = vec![];

                //iterate over all rhs columns
                for value in 0..rhs_ref.values.len() {
                    let mut c = T::default();
                    for i in 0..row.len() {
                        c = c + (row[i] * rhs_ref.values[value][i]);
                    }
                    out_row.push(c);
                }

                let mut output = output.lock().unwrap();
                output[index] = out_row;

            });


            threads.push(thread);
        }

        for thread in threads {
            thread.join().unwrap();
        }

        let vec = output_values.lock().unwrap();

        let mat = Matrix::from_vec(&vec).unwrap();

        return Ok(mat)
    }
}

//Addition and Subtraction
impl<T: 'static + Default + Copy + Send + Sync + std::ops::Add + Add<Output = T> + std::ops::Sub + Sub<Output = T>> Matrix<T> {
    pub fn addition(&self, mat: &Matrix<T>) -> Result<Matrix<T>, String>{
        if self.dimensions != mat.dimensions {
            return Err("Cannot perform addition on matricies of different dimension".to_string());
        }
        let output_mat: Matrix<T> = Matrix::new(self.dimensions.0, self.dimensions.1);

        let mat1 = Arc::new(self.clone());
        let mat2 = Arc::new(mat.clone());

        let output_mat = Arc::new(Mutex::new(output_mat));

        let mut threads: Vec<JoinHandle<()>> = vec![];
        for row in 0..self.dimensions.0 {
            let mat1_clone = mat1.clone();
            let mat2_clone = mat2.clone();

            let output_mat_clone = output_mat.clone();

            let thread = thread::spawn(move || {

                let mut thread_row = mat1_clone.values[row].clone();

                for i in 0..mat1_clone.dimensions.1 {
                    thread_row[i] = thread_row[i] + mat2_clone.values[row][i];
                }

                let mut output_mut = output_mat_clone.lock().unwrap();

                output_mut.values[row] = thread_row;
            });

            threads.push(thread);
        }

        for thread in threads {
            thread.join().unwrap();
        }

        let output = output_mat.lock().unwrap();
        Ok(output.clone())
    }

    pub fn subtraction(&self, mat: &Matrix<T>) -> Result<Matrix<T>, String>{
        if self.dimensions != mat.dimensions {
            return Err("Cannot perform subtraction on matricies of different dimension".to_string());
        }
        let output_mat: Matrix<T> = Matrix::new(self.dimensions.0, self.dimensions.1);

        let mat1 = Arc::new(self.clone());
        let mat2 = Arc::new(mat.clone());

        let output_mat = Arc::new(Mutex::new(output_mat));

        let mut threads: Vec<JoinHandle<()>> = vec![];
        for row in 0..self.dimensions.0 {
            let mat1_clone = mat1.clone();
            let mat2_clone = mat2.clone();

            let output_mat_clone = output_mat.clone();

            let thread = thread::spawn(move || {

                let mut thread_row = mat1_clone.values[row].clone();

                for i in 0..mat1_clone.dimensions.1 {
                    thread_row[i] = thread_row[i] - mat2_clone.values[row][i];
                }

                let mut output_mut = output_mat_clone.lock().unwrap();

                output_mut.values[row] = thread_row;
            });

            threads.push(thread);
        }

        for thread in threads {
            thread.join().unwrap();
        }

        let output = output_mat.lock().unwrap();
        Ok(output.clone())
    }
    
}

//prints the matrix to terminal
impl<T: std::fmt::Display> Matrix<T> {
    pub fn print(&self) {
        for y in 0..self.dimensions.0 {
            println!();
            print!("[");
            for x in 0..self.dimensions.1 {
                print!(" {},", self.values[y][x]);
            }
            print!("]");
        }
        println!();
    }
}

//non important functions used in callable functions
fn valid_size<T>(values: &Vec<Vec<T>>) -> bool {
    //check if each row is the same size
    let base_len = values[0].len();
    for i in 0..values.len() {
        if base_len != values[i].len() {
            return false; 
        }
    }

    true
}

//std::ops definitions 
impl<T: Default + Copy + Send + Sync + 'static + std::ops::Mul + Add<Output = T> + Mul<Output = T>> Mul for &Matrix<T> {
    type Output = Result<Matrix<T>, String>;

    fn mul(self, rhs: &Matrix<T>) -> Result<Matrix<T>, String>  {
        self.multiply(rhs)
    }
}

impl<T: 'static + Default + Copy + Send + Sync + std::ops::Add + Add<Output = T> + std::ops::Sub + Sub<Output = T>> Add for &Matrix<T> {
    type Output = Result<Matrix<T>, String>;

    fn add(self, rhs: &Matrix<T>) -> Result<Matrix<T>, String> {
        self.addition(rhs)
    }
}

impl<T: 'static + Default + Copy + Send + Sync + std::ops::Add + Add<Output = T> + std::ops::Sub + Sub<Output = T>> Sub for &Matrix<T> {
    type Output = Result<Matrix<T>, String>;

    fn sub(self, rhs: &Matrix<T>) -> Result<Matrix<T>, String> {
        self.subtraction(rhs)
    }
}