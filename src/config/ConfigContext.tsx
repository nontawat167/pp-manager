import { createContext, PropsWithChildren, useContext } from 'react';
import appConfig from './appConfig';

interface IAppConfig {
  version: string;
}

const AppConfigContext = createContext<IAppConfig>(appConfig);

const AppConfigProvider = ({ children }: PropsWithChildren) => (
  <AppConfigContext.Provider value={appConfig}>
    {children}
  </AppConfigContext.Provider>
);

export const useAppConfig = () => {
  const config = useContext(AppConfigContext);
  return config;
};

export default AppConfigProvider;
