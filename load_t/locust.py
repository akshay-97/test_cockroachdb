from locust import HttpUser, TaskSet, SequentialTaskSet, task,  between
config_set = {
    'count' : 0,
    'prefix' : "asdas124Poo92032"
}

class PaymentsBehaviour(SequentialTaskSet):
    payment_id = None
    @task(1)
    def payments_create(self):
        config_set['count'] += 1
        self.payment_id = config_set['prefix'] + str(config_set['count'])
        response = self.client.get('/payment_init/' + self.payment_id, name = "payment init")

    @task(1)
    def get(self):
        response = self.client.get('/show_payment/' + self.payment_id, name = "payment show")

    @task(1)
    def update(self):
        response = self.client.get('/update_payment/' + self.payment_id, name = "payment update")
    

class WebsiteUser(HttpUser):
    tasks = [PaymentsBehaviour]
