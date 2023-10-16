import initSync from './minesweeper.js';
import { FIREBASECONFIG } from './firebaseConfig.js';
const encodedValue  = FIREBASECONFIG;
const decodedString = atob(encodedValue);
const firebaseConfig = JSON.parse(decodedString);
console.log(firebaseConfig);

// Initialize Firebase
const app = firebase.initializeApp(firebaseConfig);

// Initialize Cloud Firestore and get a reference to the service
const db = firebase.firestore(app);

async function main() {
    try {
        await initSync();
    } catch (e) {
        console.error('Error in initSync:', e);
    } finally {
        document.getElementById('loader').remove();
    }
}

window.addEventListener('contextmenu', function(e) {
    e.preventDefault();
}, false);

window.listen_login_js = function(uuid) {
    return new Promise((resolve, reject) => {
        const loginDocRef = db.collection('login').doc(uuid);
        
        const unsubscribe = loginDocRef.onSnapshot((docSnapshot) => {
            if (docSnapshot.exists) {
                const userData = docSnapshot.data();
                const userId = userData.user_id;
                console.log("User ID from updated document:", userId);
                unsubscribe(); // 문서가 감지되면 리스너를 해제

                resolve(userId); // 문서가 존재하면 userId로 resolve
            } else {
                console.log("Document does not exist yet."); 
                // 아직 문서가 없으므로 대기하면서 계속 리스너를 유지
            }
        }, (error) => {
            console.error("Error listening to document:", error);
            unsubscribe(); // 에러 발생 시 리스너를 해제
            reject(new Error("Error listening to document")); // 오류가 발생했기 때문에 reject
        });
    });
}

window.add_ranking_js = function(id, time, difficulty) {
    return new Promise((resolve, reject) => {
        try {
            db.collection("ranking").add({
                id: id,
                time: time,
                difficulty: difficulty,
                created_at: firebase.firestore.FieldValue.serverTimestamp(),
            });

            resolve();
        } catch (e) {
            console.error("Error adding document: ", e);
            reject(new Error("Error adding document"));
        }
    });
}

function isValidObject(data) {
    // id가 string 타입인지 확인
    const isIdString = typeof data.id === 'string';

    // time이 float인지 확인 (JavaScript에서 모든 숫자는 float로 처리되므로 'number'로 확인)
    const isTimeFloat = typeof data.time === 'number';

    // difficulty가 string 타입인지 확인
    const isDifficultyString = typeof data.difficulty === 'string';

    // created_at이 firestore timestamp 타입인지 확인 (toDate 메서드가 존재하는지 확인)
    const isCreatedAtTimestamp = data.created_at && typeof data.created_at.toDate === 'function';

    return isIdString && isTimeFloat && isDifficultyString && isCreatedAtTimestamp;
}

window.read_ranking_js = function() {
    return new Promise((resolve, reject) => {
        const result = [];
        try {
            db.collection("ranking").get().then((querySnapshot) => {
                querySnapshot.forEach((doc) => {
                    const data = doc.data();
                    const currentSeconds = Math.floor(Date.now() / 1000);
                    if (!isValidObject(data)) {
                        console.error("Invalid object:", data);
                        return;
                    }
                    const obj = {
                        id: data.id,
                        time: data.time,
                        difficulty: data.difficulty,
                        created_at: data.created_at ? data.created_at.seconds : currentSeconds
                    };
                    result.push(obj);
                });

                resolve(result);
            });
        } catch (e) {
            console.error("Error read document: ", e);
            reject(new Error("Error read document"));
        }
    })
}


main();