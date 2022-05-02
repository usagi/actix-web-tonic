/// parse command line arguments
/// exec-arg[1] -> addr, default = "[::1]"
/// exec-arg[2] -> addr, default = 50000
/// exec-arg[3] -> addr, default = 50001
pub fn parse() -> (String, u16, u16)
{
 let args = std::env::args().collect::<Vec<_>>();

 (
  // arg[1] -> addr
  args
   .get(1)
   .map(|s| s.clone())
   .unwrap_or("[::1]".to_string()),
  // arg[2] -> port_actix
  args
   .get(2)
   .and_then(|s| {
    s.parse::<u16>()
     .ok()
   })
   .unwrap_or(50000),
  // arg[3] -> addr
  args
   .get(3)
   .and_then(|s| {
    s.parse::<u16>()
     .ok()
   })
   .unwrap_or(50001)
 )
}
