fn reverse(arr: &mut [i32], i:i32 , j:i32){
    while(i<j){
        let mut temp;
        temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
        i+=1;
        j-=1;
    }
}

fn main(){
    let mut arr:[i32;5] = [1,2,3,4,5]   // variables and array are immutabls by default but to make them mutable we can use the mut keyword
    reverse(&mut arr,0,4);
    reverse(&mut arr, 3, 4);
    reverse(&mut arr, 0 , 3);
}