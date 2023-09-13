import React from 'react';
import './styles.css'; // Import the CSS file if it's separate
import Link from '@docusaurus/Link';
import HappyVeryUrl from '@site/static/img/happy_very.png';

import ShyUrl from '@site/static/img/shy.png';
import TongueUrl from '@site/static/img/tongue.png';

function HeroFeatures() {
  return (
    <div className="hero-features-section">
      <ul>
          <li>
              <img src={HappyVeryUrl} />
              <span>Scripts</span>
          </li>
          <li>
              <img src={ShyUrl} />
              <span>Workflows</span>
          </li>
          <li>
              <img src={TongueUrl} />
              <span>Shortcuts</span>
          </li>
        
      </ul>

    </div>
  );
}

export default HeroFeatures;
