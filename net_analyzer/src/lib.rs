use tracert;
use std::net::IpAddr;

//pub fn test_trace() { 
//    let ip_tracer:Result<tracert::trace::Tracer, String> = tracert::trace::Tracer::new(TEST_IP);
//    match ip_tracer {
//        Ok(v) => {
//            println!("Tracer Setup Successfull");
//            let ip_trace:Result<tracert::trace::TraceResult, String> = v.trace();
//            match ip_trace{
//                Ok(v) => {
//                    for ip_node in v.nodes {
//                        println!("+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=");
//                        println!("Node: {}", ip_node.seq);
//                        println!("IP Address: {}", ip_node.ip_addr);
//                    }
//                    println!("Test OK!");
//                },
//                Err(e) => println!("Trace Failed. Error: {e:?}"),
//            }
//        },
//        Err(e) => println!("Tracer Setup Failed. Error: {e:?}"),
//    }
//}
//
pub fn scan(){

}

pub fn trace(dest_ip:IpAddr) -> Result<Vec<IpAddr>, String>{
    let mut path_to_dest = vec![];
    let ip_tracer:Result<tracert::trace::Tracer, String> = tracert::trace::Tracer::new(dest_ip);
    match ip_tracer {
        Ok(t) => {
            println!("Tracer Setup Successfull");
            let ip_trace:Result<tracert::trace::TraceResult, String> = t.trace();
            match ip_trace{
                
                Ok(r) => {
                    for ip_node in r.nodes {
                        //let dummy: &mut Vec<IpAddr> = &mut vec![ip_node.ip_addr];
                        path_to_dest.push(ip_node.ip_addr);
                    }
                    println!("Test OK!");
                },
                Err(e) => return Result::Err(e),
            }
        },
        Err(e) => return Result::Err(e),
    }
    return Result::Ok(path_to_dest);
}