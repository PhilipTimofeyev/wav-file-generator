import init, { build_wav } from "./wav_file_generator.js";
await init();  

const generateBtn = document.getElementById('generate')
const downloadBtn = document.getElementById('download')

generateBtn.addEventListener('click', function() {
    createFile()
})

function createFile() {
    const length = parseInt(document.getElementById('length').value)
    const frequency = document.getElementById('frequency').value

    if (!validateLength(length)) return
    if (!validateFrequency(frequency)) return

    const wavBytes = new Uint8Array(build_wav(BigInt(length), parseFloat(frequency)))
    const file = new Blob([wavBytes], { type: 'application/octet-stream' });

    const filename = `${length} sec ${frequency} Hz sine`
    downloadFile(file, filename)
}

function downloadFile(file, filename) {
        const link = document.createElement('a');
        link.href = URL.createObjectURL(file);   
        link.download = `${filename}.wav`;
        link.style.display = 'none';
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
        URL.revokeObjectURL(link.href);
}

function validateLength(length) {
    if (!length || length <= 0 || length > 300) {
        alert("Please enter a length between 1 and 300 seconds") 
    } else return true
}

function validateFrequency(frequency) {
    if (!frequency ||  frequency <= 0 || frequency >= 22050){
        alert("Please enter a frequency between 1 and 22,050 Hz")
        return false
    } else return true
}        