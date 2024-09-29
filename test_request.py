import requests
import threading

#Get requests
def test_get_request(ip_address, port, request_id, id):
    try:
        full_url = f"http://{ip_address}:{port}/api/tests?id={id}"
        
        response = requests.get(full_url)
        
        if response.status_code == 200:
            print(f"Request ID {request_id}: Request successful!")
            print(f"Request ID {request_id}: Response Code: {response.status_code}")
            print(f"Request ID {request_id}: Response Content: \n{response.text}")
        else:
            print(f"Request ID {request_id}: Failed. HTTP Status Code: {response.status_code}")
    
    except requests.exceptions.RequestException as e:
        print(f"Thread ID {request_id}: An error occurred: {e}")

# Funcion para crear y ejecutar threads concurrentes
def run_concurrent_tests(ip_address, port, num_threads, id):
    threads = []
    #crea el numero de threads deseado
    for i in range(num_threads):
        thread = threading.Thread(target=test_get_request, args=(ip_address, port, i, id))
        threads.append(thread)
        thread.start()

    #Finaliza los threads
    for thread in threads:
        thread.join()

if __name__ == "__main__":
    test_ip = "127.0.0.1"  # Localhost IP address
    test_port = 7878  # Port
    num_threads = int(input("Enter the number of concurrent requests: "))  
    id = int(input("Enter the id of the request: "))  

    run_concurrent_tests(test_ip, test_port, num_threads, id)

