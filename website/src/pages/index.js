import React from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import TechnologiesSection from '@site/src/components/Techs';
import styles from './index.module.css';
import HeroFeatures from '@site/src/components/HeroFeatures';
import Terminal from '@site/src/components/Terminal';
import UseCases from '@site/src/components/UseCases';

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <header className={clsx('hero hero--secondary', styles.heroBanner)}>
      <div className="container center">
      
          <h1 className={styles.Title}>
            <div className={styles.Line}>
              <span>CMD&nbsp;</span>
              <span className={styles.important}>Knowledge&nbsp;</span>
              <span className={styles.fullsize}>/ </span>
            </div>
            <div className={styles.SecondLine}>
              <div className={styles.TaglineBlock}>
                <div className={styles.Line}>
                  {`{`}
                </div>
                <p className={styles.Tagline}>
                  Scripts, <br />Workflows <br />&amp; Aliases 
                </p>
                <div className={styles.Line}>
                  {`}`}
                </div>
              </div>
              <div className={styles.Line}>
                {/* <span></span> */}
                <span className={styles.important}>as&nbsp; Code</span>
              </div>
            </div>
          </h1>


        {/* <h1 className="hero__title">{siteConfig.title}</h1> */}
        {/* <div className={styles.titleWrapper}>
          <div className={styles.title2}>
            Terminal
          </div>
          <div className={styles.title}>
            Knowledge
          </div>
          <div className={styles.title2}>
            As Code
          </div>
        </div> */}

        <div className={null}>
          <UseCases/>
        </div>
        <h2>
          Install with one line:
        </h2>
        <div className={styles.terminal}>
          <Terminal text={"bash -c \"$(curl -fsSL https://raw.githubusercontent.com/mzsrtgzr2/cocmd/main/install.sh)\""}/>
        </div>

        {/* <div className={styles.terminal2}>
          <Terminal text={"pip install cocmd"}/>
        </div> */}
        
      </div>
    </header>
  );
}

export default function Home() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title={`${siteConfig.title}`}
      description="community for bash aliases, scripts and automations">
      <HomepageHeader />
      {/* <HeroFeatures/> */}

      {/* <div className={styles.buttons}>
          <Link
            className="button button--primary button--lg"
            to="/docs/intro">
            Read the Docs
          </Link>
      </div> */}

      <main>

        <TechnologiesSection />
      </main>
      {/* <a href="https://www.flaticon.com/free-icons/leadership" title="leadership icons">Leadership icons created by HAJICON - Flaticon</a> */}
    </Layout>
  );
}
