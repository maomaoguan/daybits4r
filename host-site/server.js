const express = require('express');
const path = require('path');

const app = express();

app.use(express.static(path.join(__dirname, 'static')));

app.get('/health', (req, res) => {
  res.json({ ok: true });
});

const port = process.env.PORT ? parseInt(process.env.PORT, 10) : 4000;
app.listen(port, () => {
  console.log(`Host site listening on http://localhost:${port}`);
});

