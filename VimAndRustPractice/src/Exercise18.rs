use tokio::time::{sleep, Duration};  

pub async fn setupAsynchPractice() {
    tokioPractice().await;
}

async fn tokioPractice() {
    let task1 = async {  
        println!("Task 1 started!");  
        sleep(Duration::from_secs(2)).await;  
        println!("Task 1 completed!");  
    };  

    let task2 = async {  
        println!("Task 2 started!");  
        sleep(Duration::from_secs(1)).await;  
        println!("Task 2 completed!");  
    };  

    tokio::join!(task1, task2); 
}