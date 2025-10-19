struct gy{
    chunksize:u8,
    
}



fn chunk(story:&str){
    let mut b=String::from("");
       for ty in story.chars(){
       if !ty.is_ascii_whitespace(){
      if b.len()<=4{
        b.push(ty);
       }else {
           println!("{}",b);
            b.clear();
             b.push(ty);
       }
    }
      }
      
}


fn main() {
    let  story="hello world nbsaidbf njbaigfbir jiubasdiubfgu uahsfgiberiq buuahsfu bibaeif ";
    chunk(story);
   
}
