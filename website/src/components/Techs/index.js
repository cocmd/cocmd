import React from 'react';
import './styles.css'; // Import the CSS file if it's separate
import Link from '@docusaurus/Link';

function TechnologiesSection() {
  const technologies = [
    { name: 'Docker', logo: 'https://uxwing.com/wp-content/themes/uxwing/download/brands-and-social-media/docker-icon.png', link: 'docs/technologies/docker' },
    { name: 'Git', logo: 'https://upload.wikimedia.org/wikipedia/commons/thumb/3/3f/Git_icon.svg/2048px-Git_icon.svg.png', link: 'docs/technologies/git' },
    { name: 'Kubernetes', logo: 'https://user-images.githubusercontent.com/19824574/41482054-47a3a702-70a2-11e8-9561-de51c5f71220.png', link: 'docs/technologies/kubernetes' },
    { name: 'AWS CLI', logo: 'https://cdn.iconscout.com/icon/free/png-256/free-aws-1869025-1583149.png', link: 'docs/technologies/awscli' },
    { name: 'Terraform', logo: 'https://static-00.iconduck.com/assets.00/terraform-icon-452x512-ildgg5fd.png', link: 'docs/technologies/terraform' },
    { name: 'Ansible', logo: 'https://www.ansible.com/hubfs/2016_Images/Assets/Ansible-Mark-Large-RGB-Pool.png', link: 'docs/technologies/ansible' },
    { name: 'Jenkins', logo: 'https://www.jenkins.io/images/logos/jenkins/jenkins.png', link: 'docs/technologies/jenkins' },
    { name: 'Python', logo: 'https://icons-for-free.com/iconfiles/png/512/super+tiny+icons+python-1324450764865983278.png', link: 'docs/technologies/python' },
    { name: 'Node.js', logo: 'https://static-00.iconduck.com/assets.00/node-js-icon-454x512-nztofx17.png', link: 'docs/technologies/nodejs' },
    { name: 'Grafana', logo: 'https://grafana.com/static/assets/img/apple-touch-icon.png', link: 'docs/technologies/grafana' },
    { name: 'Prometheus', logo: 'https://upload.wikimedia.org/wikipedia/commons/thumb/3/38/Prometheus_software_logo.svg/2066px-Prometheus_software_logo.svg.png', link: 'docs/technologies/prometheus' },
    { name: 'Elasticsearch', logo: 'https://w7.pngwing.com/pngs/675/954/png-transparent-elasticsearch-kibana-logo-logstash-business-business-people-logo-computer-wallpaper-thumbnail.png', link: 'docs/technologies/elasticsearch' },
    // Add more technologies here
  ];

  return (
    <div className="technologies-section">
      <h2>Get Shortcuts and Scripts <br/>for 800+ Technologies</h2>
      <ul>
        {technologies.map((tech, index) => (
          <li key={index}>
            <a href={tech.link}>
              <img src={tech.logo} alt={`${tech.name} logo`} />
              <span>{tech.name}</span>
            </a>
          </li>
        ))}
      </ul>

      <div className="create-own">
      <Link
            className="button button--primary button--lg"
            to="/docs/intro">
            or Create your own
          </Link>
          </div>
    </div>
  );
}

export default TechnologiesSection;
