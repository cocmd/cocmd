import React, { useState } from 'react';

const Terminal = ({ text }) => {
  const [isCopied, setIsCopied] = useState(false);

  const terminalStyle = {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    backgroundColor: '#ededed',
    color: '#000',
    padding: '20px',
    borderRadius: '10px',
    fontFamily: 'Courier, monospace',
    fontSize: '16px',
    lineHeight: '24px',
    width: '80%',
    margin: '0 auto',
    textAlign: 'left',
  };

  const promptStyle = {
    color: '#9861ff',
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(text).then(() => {
      setIsCopied(true);
      setTimeout(() => setIsCopied(false), 2000);
    });
  };

  return (
    <div style={terminalStyle}>
      <div>
        <span style={promptStyle}>$ </span>
        {text}
      </div>
      <button onClick={handleCopy}>
        {isCopied ? 'Copied!' : 'Copy'}
      </button>
    </div>
  );
};

export default Terminal;
