db.createUser({
  user: "lmsApi",
  pwd: "",
  roles: [
      {
          role: "readWrite",
          db: "lms-api-db"
      }
  ]
})
