const connString =
  "mongodb+srv://saiot:saiot@notifications.cab90.mongodb.net/notifications?retryWrites=true&w=majority";
const port = 3000;
const pollInterval = 60;

const express = require("express");
const bodyParser = require("body-parser");
const MongoClient = require("mongodb").MongoClient;
const app = express();

app.set("view engine", "ejs");
app.use(express.static("public"));
app.use(bodyParser.json());

MongoClient.connect(connString, { useUnifiedTopology: true })
  .then((client) => {
    console.log("Successfully connected to MongoDB.");

    const db = client.db("notifications");
    const notificationsCollection = db.collection("notifications");

    app.listen(port, function () {
      console.log("listening on " + port);
    });

    app.use(bodyParser.urlencoded({ extended: true }));

    // UI (method: GET)
    app.get("/notifications", (req, res) => {
      db.collection("notifications")
        .find()
        .toArray()
        .then((results) => {
          console.log(results);
          res.render("index.ejs", { notifications: results });
        })
        .catch((error) => console.error(error));
    });

    // UI (method: POST)
    app.post("/notifications", (req, res) => {
      notificationsCollection
        .insertOne({
          name: req.body.name,
          notification: req.body.notification,
          timestamp: (Date.now() / 1000) | 0,
        })
        .then((result) => {
          console.log(result);
          res.redirect("/notifications");
        })
        .catch((error) => console.error(error));
    });

    // UI (method: PUT)
    app.put("/notifications", (req, res) => {
      notificationsCollection
        .findOneAndUpdate(
          { name: req.body.name },
          {
            $set: {
              name: req.body.name,
              notification: req.body.notification,
              timestamp: parseInt(req.body.timestamp),
            },
          },
          {
            upsert: true,
            returnNewDocument: true,
          }
        )
        .then((result) => {
          console.log(result);
          res.json("Success");
        })
        .catch((error) => console.error(error));
    });

    // UI (method: DEL)
    app.delete("/notifications", (req, res) => {
      notificationsCollection
        .deleteOne({ name: req.body.name })
        .then((result) => {
          if (result.deletedCount === 0) {
            return res.json("No notifications to delete").status(201);
          }
          res
            .json("Deleted notification with name: " + req.body.name)
            .status(200);
        })
        .catch((error) => console.error(error));
    });

    // API (method: GET, format: JSON)
    app.get("/api/notifications", (req, res) => {
      const now = Date.now() / 1000;
      const timeRef = (now - pollInterval) | 0;
      db.collection("notifications")
        .find({
          $and: [
            { timestamp: { $gte: timeRef } },
            { timestamp: { $lte: now } },
          ],
        })
        .limit(1)
        .next()
        .then((results) => {
          console.log(results);
          res.json(results);
        })
        .catch((error) => console.error(error));
    });

    // UI (method: POST)
    app.post("/api/snooze", (req, res) => {
      console.log(req.body);

      db.collection("notifications")
        .find({ timestamp: { $lt: (Date.now() / 1000) | 0 } })
        .sort({ timestamp: -1 })
        .limit(1)
        .next()
        .then((notif) => {
          const newTimestamp = (Date.now() / 1000 + req.body.snooze) | 0;
          console.log(
            "Updating notification: " +
              notif.name +
              " from timestamp: " +
              notif.timestamp +
              " from timestamp: " +
              newTimestamp
          );

          notificationsCollection
            .findOneAndUpdate(
              { timestamp: notif.timestamp },
              {
                $set: {
                  timestamp: newTimestamp,
                },
              },
              {
                returnNewDocument: true,
              }
            )
            .then((result) => {
              console.log(result);
              res.json("Managed to change the timestamp.");
            })
            .catch((error) => res.json(error));
        })
        .catch((error) => res.json(error));
    });
  })
  .catch((error) => res.json(error));
