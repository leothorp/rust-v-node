const express = require('express');
const app = express();

app.use(express.json());


function fibonacci(n) {
    if (n <= 1) {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

app.get('/compute/fibonacci/:n', (req, res) => {
    const n = parseInt(req.params.n, 10);
    const result = fibonacci(n);
    res.send(`Fibonacci(${n}) = ${result}`);
});


app.listen(3000, () => {
    console.log('Express app listening on port 3000');
});
