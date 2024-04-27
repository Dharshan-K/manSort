use std::env;
use std::fs;

fn main() {    
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    let file_path = &args[1];
    println!("file name: {}", file_path);
    let mut content = fs::read_to_string(file_path).expect("read the file");
    let mut contentArray: Vec<&str> = content.lines().collect();
    let mut lines_vec: Vec<String> = Vec::new();
    for line in contentArray {
        lines_vec.push(line.to_string());
    }
    mergeSort(&mut lines_vec);
    println!("the contents are: {:?}", lines_vec);
    println!("Hello, world!");
}

fn mergeSort<T:Ord + Clone>(arr: &mut [T]){
    let len = arr.len();
    if len <= 1{
        return
        }

    let mid = len / 2;

    let mut leftHand = arr[..mid].to_vec();
    let mut rightHand = arr[mid..].to_vec();

    mergeSort(&mut leftHand);
    mergeSort(&mut rightHand);

    let mut i=0;
    let mut j=0;
    let mut k=0;

    while i<leftHand.len() && j<rightHand.len(){
        if leftHand[i] <= rightHand[j]{
            arr[k] = leftHand[i].clone();
            i +=1;
        }else{
            arr[k] = rightHand[j].clone();
            j+=1;
        }
        k+=1;
    }

    while i < leftHand.len(){
        arr[k] = leftHand[i].clone();
        i +=1;
        k+=1;
    }

    while j< rightHand.len(){
        arr[k] = rightHand[j].clone();
        j+=1;
        k+=1
    }
}

