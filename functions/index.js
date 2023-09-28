const {onRequest} = require("firebase-functions/v2/https");
const logger = require("firebase-functions/logger");
const admin = require("firebase-admin");
const {Timestamp} = require("firebase-admin/firestore");

admin.initializeApp();
const db = admin.firestore();

exports.uploadRanking = onRequest(async (req, res) => {
  const {nickname, difficulty, time} = req.body;
  await db.collection("ranking").add({
    nickname,
    difficulty,
    time,
    timestamp: Timestamp.now(),
  });
  res.status(200).send("Ranking added successfully!");
});

exports.readRanking = onRequest(async (req, res) => {
  const difficulty = req.query.difficulty;
  if (typeof difficulty === "undefined") {
    res.status(400).send("Difficulty is required");
    return;
  }
  const snapshot = await db.collection("ranking")
      .where("difficulty", "==", difficulty)
      .orderBy("time", "asc")
      .limit(100)
      .get();
  const results = snapshot.docs.map((doc) => doc.data());
  res.status(200).send(results);
});

exports.findNick = onRequest(async (req, res) => {
  const difficulty = req.query.difficulty;
  const nickname = req.query.nickname;
  const snapshot = await db.collection("ranking")
      .where("nickname", "==", nickname)
      .where("difficulty", "==", difficulty)
      .orderBy("time", "asc")
      .limit(100)
      .get();
  const results = snapshot.docs.map((doc) => doc.data());
  res.status(200).send(results);
});
