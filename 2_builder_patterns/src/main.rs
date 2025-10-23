

#[derive(Default,Debug)]
struct User{
    name:String,
    email:String,
    age:u8
}


impl User {
    fn build(self)->User{
        User{
            name: self.name,
            email: self.email,
            age: self.age,
        }
    }

    fn name(mut self,name:&str)->Self{
          self.name=name.to_string();
          self
        
    }

     fn email(mut self,email:&str)->Self{
        self.email=email.to_string();
        self
     }

     fn age(mut self,age:u8)->Self{
        self.age=age;
        self
     }
     
}

fn main() {
    let builder=User::default();
   let  u1=builder.name("Raushan").email("raushan@gmail.com").age(21).build();
  println!("User:{:?}",u1);

}
