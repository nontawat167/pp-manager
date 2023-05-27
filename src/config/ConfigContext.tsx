import { createContext, PropsWithChildren, useContext, useEffect } from 'react';
import appConfig from './appConfig';

type AppConfig = typeof appConfig;

const AppConfigContext = createContext<AppConfig>(appConfig);

const disableContextMenu = (e: Event) => {
  e.preventDefault();
};

const AppConfigProvider = ({ children }: PropsWithChildren) => {
  useEffect(() => {
    const { enabled, hostname } = appConfig.contextMemu;
    if (enabled || !hostname.includes(window.location.hostname)) {
      return;
    }
    document.addEventListener('contextmenu', disableContextMenu);

    // cleanup this component
    return () => {
      document.removeEventListener('contextmenu', disableContextMenu);
    };
  }, []);
  return (
    <AppConfigContext.Provider value={appConfig}>
      {children}
    </AppConfigContext.Provider>
  );
};

export const useAppConfig = () => {
  const config = useContext(AppConfigContext);
  return config;
};

export default AppConfigProvider;
