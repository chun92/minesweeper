const {onRequest} = require("firebase-functions/v2/https");
const admin = require("firebase-admin");
const {Timestamp} = require("firebase-admin/firestore");
const axios = require("axios");
const fs = require("fs");
const yaml = require("js-yaml");
const {schedule} = require("firebase-functions/v1/pubsub");

admin.initializeApp();
const db = admin.firestore();

exports.oauth = onRequest(async (request, response) => {
  const state = request.query.state;
  if (!state) {
    response.status(400).send("Missing state parameter");
    return;
  }

  const code = request.query.code;
  if (!code) {
    response.status(400).send("Missing auth code");
    return;
  }

  try {
    const configFile = fs.readFileSync("config.yaml", "utf8");
    const config = yaml.load(configFile);

    const clientId = config.settings.client_id;
    const clientSecret = config.settings.client_secret;
    const redirectUri = config.settings.redirect_uri;

    const tokenResponse = await axios.post("https://oauth2.googleapis.com/token", {
      code: code,
      client_id: clientId,
      client_secret: clientSecret,
      redirect_uri: redirectUri,
      grant_type: "authorization_code",
    });
    const accessToken = tokenResponse.data.access_token;
    const userInfoResponse = await axios.get("https://www.googleapis.com/oauth2/v3/userinfo", {
      headers: {
        "Authorization": `Bearer ${accessToken}`,
      },
    });

    const email = userInfoResponse.data.email;
    const docRef = admin.firestore().collection("login").doc(state);
    await docRef.set({
      user_id: email,
      created_at: Timestamp.now(),
    });
    response.send("Successfully login. Please close this window.");
  } catch (error) {
    console.error("Error fetching user information:", error);
    response.status(500).send("Failed to fetch user information");
  }
});

exports.deleteOldItems = schedule("every 60 minutes").onRun(async (context) => {
  const oneHourAgo = new Date(Date.now() - 60 * 60 * 1000);

  const oldItemsQuery = admin.firestore()
      .collection("login")
      .where("created_at", "<", oneHourAgo);
  const snapshot = await oldItemsQuery.get();
  const deletions = snapshot.docs.map((doc) => doc.ref.delete());
  return Promise.all(deletions);
});

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
