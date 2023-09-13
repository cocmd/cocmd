import React from 'react';
import './styles.css'; // Import the CSS file if it's separate
import WorkingMan from '@site/static/img/usecases/working.png';
import BusinessMeeting from '@site/static/img/usecases/business-meeting.png';
import User from '@site/static/img/usecases/user.png';
import Leadership from '@site/static/img/usecases/leadership.png';

function UseCases() {
  const useCases = [
    { name: 'Sharable Scripts & Aliases', logo: Leadership, link: 'docs/useCases/kubernetes' },
    // { name: 'Sharable Aliases', logo: BusinessMeeting, link: 'docs/useCases/kubernetes' },
    { name: 'Onboarding a new employee', logo: WorkingMan, link: 'docs/useCases/docker' },
    { name: 'On call prodecures', logo: User, link: 'docs/useCases/git' },
    { name: 'Team knowledge as Code', logo: BusinessMeeting, link: 'docs/useCases/kubernetes' },
    
    
    // Add more useCases here
  ];

  return (
    <div className="useCases-section">
      {/* <h1>What can it do?</h1> */}
      <ul>
        {useCases.map((tech, index) => (
          <li key={index}>
            <a href={tech.link}>
              <img src={tech.logo} alt={`${tech.name} logo`} />
              <span>{tech.name}</span>
            </a>
          </li>
        ))}
      </ul>

    </div>
  );
}

export default UseCases;
