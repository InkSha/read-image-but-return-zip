import express from 'express'
import multer from 'multer'
import fs from 'node:fs'

const app = express()
const port = 3500
const upload_dir = 'uploads'

const upload = multer({
  dest: upload_dir,
  limits: {
    fileSize: 500 * 1024 * 1024
  },
  storage: multer.diskStorage({
    destination: (req, file, cb) => {
      console.log('destination', file.originalname)
      cb(null, upload_dir)
    },
    filename: (req, file, cb) => {
      console.log('filename', file.originalname)
      cb(null, file.originalname)
    }
  })
})

app.post('/upload',
  (req, res, next) => {
    console.log('enter upload')
    next()
  },
  upload.single('file'),
  (req, res) => {
    console.log(req.body)
    console.log(req.file)
    res.send('File uploaded successfully')
  }
)

app.listen(port, () => {
  if (fs.existsSync(upload_dir)) {
    fs.rmSync(upload_dir, { recursive: true })
  }
  fs.mkdirSync(upload_dir)
  console.log(`app listening on port ${port}`)
})
