import init, { tokenize } from '../wasm/web_tokenizers.js';

onmessage = async (event) => {
    const { id, text } = event.data;

    await init();

    const result = tokenize(text);

    postMessage({
        id,
        data: {
            tokens: result.tokens,
            inputIds: result.inputIds,
            attentionMask: result.attentionMask,
        }
    });
}