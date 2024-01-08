use solana_program_test::tokio;



#[tokio::test]
async fn random() {

  let find = [0, 0, 0, 0];
  let n = 8;
 
  let rand: f32 = n as f32 / 10.0; // рандом
  // let f: f32 = (rand * ((find.len()) as f32) ) as f32;
  // let mut index = f.floor();

  // if index as usize > find.len() - 1 {
  //   index = (find.len() - 1) as f32;
  // }

  println!("random - {}", rand);

}