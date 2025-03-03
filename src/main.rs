use LinkedList::*;

#[derive(Debug)]
enum LinkedList<T>{
    Cur(T,Box<LinkedList<T>>),
    Nil,
}

impl<T> LinkedList<T> {
    fn new()->LinkedList<T>{
        LinkedList::Nil
    }
    fn add(self,val:T)->LinkedList<T>{
        LinkedList::Cur(val,Box::new(self))
    }
    fn len(&self)->usize{
        let mut size: usize =0;
        let mut cur = self;
        loop{
            match cur{
                Cur(_,next)=>{
                    cur = next;
                    size+=1;
                },
                Nil=>break,
            }
        }
        size
    }
    fn print<F>(&self,fun_print:F)
    where 
        F: Fn(&T),
    {
        let mut cur = self;
        loop{
            match cur{
                Cur(val,next)=>{
                    fun_print(val);
                    cur = next;
                }
                Nil=>break,
            }
        }
        println!();
    }
}

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list = list.add(10);
    list = list.add(40);
    let size = list.len();
    println!("size {}",size);
    list.print(print_i32);
}

fn print_i32(int:&i32){
    print!("{} ",int);
}