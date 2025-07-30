import Tokenizer from '../workers/tokenizer.js?worker'

const tokenizer = new Tokenizer();

let messageId = 0;
const pendingRequests = new Map();

tokenizer.onmessage = e => {
    const { id, data } = e.data;
    if (pendingRequests.has(id)) {
        pendingRequests.get(id).resolve(data);
        pendingRequests.delete(id);
    }
};

export async function encodeAsync(text) {
    let { promise, resolve, reject } = Promise.withResolvers();

    const id = ++messageId;
    pendingRequests.set(id, { resolve, reject });

    tokenizer.postMessage({ id, text });

    return promise;
}