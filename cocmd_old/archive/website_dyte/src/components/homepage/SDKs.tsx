import React from 'react';
import Link from '@docusaurus/Link';

function SDK({ icon, to, name }: { icon: string; name: string; to?: string }) {
  return (
    <Link
      to={to}
      className="flex cursor-pointer items-center rounded-lg border border-secondary-700 p-2.5 text-inherit hover:border-primary hover:text-primary hover:no-underline"
    >
      <img src={icon} className="mr-2 h-7 " />
      <span className="font-medium">{name}</span>
    </Link>
  );
}

export default function SDKs() {
  return (
    <section className="mx-auto mb-32 flex w-full max-w-5xl flex-col p-4 py-0">
      {/* <span className="mb-2 uppercase tracking-wider text-text-400">
        SDK Documentation
      </span>

      <h3 className="mb-12 text-4xl">
        Build the way you want in the framework you want!
      </h3> */}

      <div className="mb-10">
        <h4 className="mb-2 text-2xl">Scripts by Technology</h4>

        <p className="mb-6 text-text-400">
        Get goodies per technology 
        </p>

        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-4">
          <SDK
            name="Python"
            to="/python"
            icon="/static/landing-page/sdk-icons/python.png"
          />

          <SDK
            name="Git"
            to="/git"
            icon="/static/landing-page/sdk-icons/kafka.png"
          />
          <SDK
            name="Docker"
            to="/docker"
            icon="/static/landing-page/sdk-icons/spark.png"
          />
        </div>
      </div>

      <div>
        <h4 className="mb-2 text-2xl">What is your workflow?</h4>

        <p className="mb-6 text-text-400">
          Get or create automations that will make you run much faster
        </p>

        <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-4">
          <SDK
            name="Daily routine"
            to="/web-core"
            icon="/static/landing-page/sdk-icons/justified.png"
          />
          <SDK
            name="New Hire setup"
            to="/web-core"
            icon="/static/landing-page/sdk-icons/justified.png"
          />
          
        </div>
      </div>
    </section>
  );
}
