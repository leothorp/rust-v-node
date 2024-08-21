const express = require('express');
const { Pool } = require('pg');
const app = express();

app.use(express.json());

const db = new Pool({
    connectionString: process.env.DATABASE_URL
});

// Naive recursive Fibonacci function
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

app.get('/item/:id', async (req, res) => {
    const { id } = req.params;
    try {
        const result = await db.query('SELECT * FROM items WHERE id = $1', [id]);
        res.json(result.rows[0]);
    } catch (err) {
        res.status(500).send('Failed to fetch item');
    }
});

app.listen(3000, () => {
    console.log('Express app listening on port 3000');
});
