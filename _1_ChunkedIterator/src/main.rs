struct Chunked<I> {
    iter: I,
    chunk_size: usize,
}
//this is struct and its generic impl
impl<I> Chunked<I> {
    fn new(iter: I, chunk_size: usize) -> Self {
        Self { iter, chunk_size }
    }
}

// this is extention of the chunk it is generic and where clause for impl what I have some rest
// here Iterator is trait like this , trait Iterator{}, it is standard so that
impl<I> Iterator for Chunked<I>
where// here where is clause is some type of bounding on the I means any type , can you say like helping generic in some ways to dive with some type
    I: Iterator,
    I::Item:Clone,
{
    // here iterator give type and next 
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = Vec::with_capacity(self.chunk_size);

        for _ in 0..self.chunk_size {
            if let Some(item) = self.iter.next() {
                chunk.push(item);
            } else {
                break;
            }
        }

        if chunk.is_empty() {
            None 
        } else {
            Some(chunk)
        }
    }
}

// this is triat and for externsion iterator and giving the .methods
// trait Yechchr:Iterator+Sized {
//     fn chunked(self,size:usize)->Chunked<Self>{
//         Chunked::new(self,size)
//     }
// }

// impl <I:Iterator> Yechchr for I {}


fn main() {
    let story = "hello world nbsaidbf njbaigfbir jiubasdiubfgu uahsfgiberiq buuahsfu bibaeif";

    let filtered_chars = story.chars().filter(|c| !c.is_ascii_whitespace());

    let chunked = Chunked::new(filtered_chars, 5);

    for chunk in chunked {
        let s: String = chunk.into_iter().collect();
        println!("{}", s);
    }
}




// what is understand that first struct then impl with struct , then using generic in struct and generic in impl of struct 
// then using where clause boundiung type of struct like T.
// then trait is beverairol thing , here Iterator is build in trait , then it  is normal impl function for the 
// Trait , and then Iterator trait , have build type and next , so we deined inside impl block 
// in one trait we commented is like extension capabilities of the standard rust trait , "trait Yechchr:Iterator+Sized "  here this
//it means that we can extend the cap. of one triat , defined here
// then  trait can used as method .chrunked, then chunked fun knows what to do with it 
// what we cover in this is the struct , we relatively covered vast rust syntax and chpter . just docs some chp left to cover